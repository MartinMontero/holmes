# DeepSeek alias re-verification vs the guard's alias table

Date: 2026-09-07 (~15:05 PDT). Carried item: STATE.md staging obligation
#5 — "DeepSeek alias primary re-verification (api-docs.deepseek.com
egress) — carry to next open-egress session and Phase RC." Discharged
here: this host has egress.

## Primary source [VERIFIED-LIVE]

`https://api-docs.deepseek.com/quick_start/pricing` — fetched directly
with curl from this host, HTTP 200, 46,100 bytes, 2026-09-07 ~15:05 PDT.

Published current model ids on that page (regex `deepseek-[a-z0-9.-]+`
over the full HTML, counts in parentheses — code samples + param table):

| Published id | Note on the page |
|---|---|
| `deepseek-v4-flash` (x3) | "updated to DeepSeek-V4-Flash-0731"; call name unchanged |
| `deepseek-v4-pro` (x6) | "updated to DeepSeek-V4-Pro-0813"; call name unchanged |
| `deepseek-v4-flash-vision-exp` (x3) | new experimental vision model |

**`deepseek-chat` and `deepseek-reasoner` appear NOWHERE on the page**
(zero regex hits). The retirement announced for 2026-07-24 (CLAUDE.md
spec block) is confirmed at the primary source.

## The guard's table [VERIFIED-LIVE, read from source]

`crates/holmes-guard/src/policy.rs`:
- `PERMITTED_MODEL_FAMILIES`: `("deepseek", &["deepseek-v"])` — prefix
  match on the normalized id; comment: "The deepseek prefix pins the V4
  line: retired alias ids do not match it and therefore deny as unknown."
- L1a allowlist carries `api.deepseek.com:443`; BYOK seam
  `DEEPSEEK_API_KEY` (spawn.rs).
- `tests/acdl1_resolution.rs::s2_retired_deepseek_alias_ids_do_not_resolve`
  constructs `deepseek-chat` / `deepseek-reasoner` at runtime and
  asserts both deny as `UnknownModel` (literals deliberately uncommitted).

## Comparison — drift verdict

| Provider list (2026-09-07) | Guard posture | Verdict |
|---|---|---|
| `deepseek-v4-flash` | matches prefix `deepseek-v` → permitted | CONSISTENT |
| `deepseek-v4-pro` | matches prefix → permitted | CONSISTENT |
| `deepseek-v4-flash-vision-exp` | matches prefix → permitted | CONSISTENT (new since the table was written; the prefix family absorbs it by design) |
| `deepseek-chat` (retired) | denies as unknown (tested) | CONSISTENT — retirement confirmed |
| `deepseek-reasoner` (retired) | denies as unknown (tested) | CONSISTENT — retirement confirmed |

**NO DRIFT in the guard. Nothing changed, nothing silently edited.**

## Caveats (labeled)

- The authoritative list is `GET https://api.deepseek.com/models`, which
  requires an API key. No DeepSeek key exists on this host and none was
  requested. The docs page is the published model list; the `/models`
  endpoint cross-check remains **UNVERIFIED** — run it on any host with
  a key: `curl https://api.deepseek.com/models -H "Authorization:
  Bearer $MY_DEEPSEEK_API_KEY"` and confirm no `deepseek-chat` /
  `deepseek-reasoner` id is returned.
- One new wrinkle worth a D-item at Phase RC, flagged not fixed:
  `deepseek-v4-flash-vision-exp` accepts **image input** — a new
  modality inside a permitted prefix. The guard governs provider/model
  identity, not modality; whether the beta wants a vision-capable id
  permitted is a policy question, not a code defect. [ASSUMED —
  relevance; the id is live on the pricing page, VERIFIED-LIVE.]
- The `[NEEDS-CAVEAT]` in CLAUDE.md on DeepSeek Pro **discount
  permanence** is a pricing question; the pricing page was fetched but
  pricing-permanence is not provable from a current snapshot. Still
  caveat-grade; budget both rates. Not re-verified here.
