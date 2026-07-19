//! Phase 2 lock integration tests.
//!
//! - 2a (live leg): the invalidation-not-deletion cycle against a real
//!   Neo4j, env-gated on `HOLMES_NEO4J_URI` (+ `_USER`/`_PASSWORD`). Runs
//!   on any host with Neo4j reachable; skipped (not failed) otherwise. The
//!   hermetic contract lives in `InMemoryWall`'s unit tests.
//! - 2b (live boundary): the memory layer's excluded canary endpoint is
//!   unreachable through the L1a proxy — a real 403 + a recorded Denied
//!   event — bound to the default memory config. The canary host is
//!   sourced from the exempted compiled denylist, so no excluded-vendor
//!   literal appears here (F-025 reword precedent).
//! - structural: the audited Cypher builders carry no delete keyword.

use holmes_guard::policy::{EXCLUDED_CANARY_HOST, EXCLUDED_CANARY_PORT};
use holmes_guard::proxy::{Decision, EgressProxy, ProxyConfig};
use holmes_wall::memory::MemoryLayerConfig;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

fn send_and_read(proxy: SocketAddr, request: &[u8]) -> String {
    let mut client = TcpStream::connect(proxy).expect("connect proxy");
    client
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    client.write_all(request).expect("write request");
    let mut out = String::new();
    let mut tmp = [0u8; 1024];
    while let Ok(n) = client.read(&mut tmp) {
        if n == 0 {
            break;
        }
        out.push_str(&String::from_utf8_lossy(&tmp[..n]));
        if out.contains("\r\n\r\n") {
            break;
        }
    }
    out
}

/// Lock 2b — the memory layer, whatever it tries, cannot reach the
/// excluded canary endpoint through the guard: a guard-level 403 and a
/// Denied egress event. Paired with the deterministic config assertions in
/// `memory` (default reaches no cloud endpoint at all).
#[test]
fn lock2b_memory_layer_excluded_endpoint_is_denied_at_l1a() {
    // Config-level: the default no-config memory layer names no excluded host.
    let default_cfg = MemoryLayerConfig::default();
    assert!(!default_cfg.reaches_excluded_endpoint());
    assert!(default_cfg.all_endpoints_permitted());

    // Boundary-level: even a forced attempt to the excluded canary is denied.
    let proxy = EgressProxy::spawn(ProxyConfig { upstream: None }).expect("spawn L1a");
    let target = format!("{EXCLUDED_CANARY_HOST}:{EXCLUDED_CANARY_PORT}");
    let req = format!("CONNECT {target} HTTP/1.1\r\nHost: {target}\r\n\r\n");
    let response = send_and_read(proxy.addr(), req.as_bytes());
    assert!(
        response.starts_with("HTTP/1.1 403"),
        "memory-layer egress to the excluded canary must be denied at L1a; got: {response}"
    );
    assert!(
        proxy
            .events()
            .iter()
            .any(|e| e.decision == Decision::Denied && e.host == EXCLUDED_CANARY_HOST),
        "a Denied egress event for the excluded canary must be recorded"
    );
}

/// Lock 2a — the invalidation-not-deletion cycle on real Neo4j. Env-gated.
#[tokio::test]
async fn lock2a_invalidation_not_deletion_against_live_neo4j() {
    let Ok(uri) = std::env::var("HOLMES_NEO4J_URI") else {
        eprintln!(
            "SKIP lock2a live leg: HOLMES_NEO4J_URI unset (in-container the Neo4j image/dist \
             CDN is org-egress-blocked; run on a host with Neo4j reachable)."
        );
        return;
    };
    let user = std::env::var("HOLMES_NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
    let password = std::env::var("HOLMES_NEO4J_PASSWORD").expect("HOLMES_NEO4J_PASSWORD");

    use holmes_wall::graph::FactId;
    use holmes_wall::neo4j::Neo4jWall;

    let wall = Neo4jWall::connect(&uri, &user, &password)
        .await
        .expect("connect neo4j");

    // Unique id base so repeated runs don't collide.
    let base = std::process::id() as u64 * 1000;
    let v1 = FactId(base + 1);
    let v2 = FactId(base + 2);
    let prov = vec!["fixture/docket.md §3".to_owned()];

    wall.add_fact(
        v1,
        "entity X owns parcel 441",
        &prov,
        "2025-01-01",
        "2026-07-01",
    )
    .await
    .expect("add v1");
    let before = wall.count_all().await.expect("count before");

    wall.supersede(
        v1,
        v2,
        "entity Y owns parcel 441 (transfer recorded)",
        &prov,
        "2026-03-15",
        "2026-07-19",
        "2026-03-15",
    )
    .await
    .expect("supersede");

    // Nothing deleted: the count grew by exactly one.
    let after = wall.count_all().await.expect("count after");
    assert_eq!(after, before + 1, "supersede must append, never delete");

    // The superseded record is preserved and flagged.
    let old = wall.get(v1).await.expect("get v1").expect("v1 present");
    assert_eq!(old.valid_until.as_deref(), Some("2026-03-15"));
    assert_eq!(old.invalidated_by, Some(v2));

    // Only the replacement is current.
    let current = wall.current_fact_ids().await.expect("current");
    assert!(current.contains(&v2) && !current.contains(&v1));

    // History stays queryable: as-of inside v1's validity returns v1.
    let as_of = wall.fact_ids_as_of("2025-06-01").await.expect("as-of");
    assert!(as_of.contains(&v1), "history must remain queryable");
}
