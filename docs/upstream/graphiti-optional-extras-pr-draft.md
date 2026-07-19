# Upstream PR draft — graphiti: make `openai` and `posthog` optional extras

**Status:** DRAFT for Martin to file against `getzep/graphiti` (D-12 rider e).
This session's GitHub access is scoped to `MartinMontero/holmes`, so the fork/PR
cannot be opened from here; the change is drafted below. No Holmes schedule
dependency rides on its acceptance (the owned subset proceeds regardless).

## Title

Make `openai` and `posthog` optional extras

## Body

`graphiti-core`'s `pyproject.toml` lists `openai>=1.91.0` and `posthog>=3.0.0`
as unconditional base dependencies (observed on 0.29.2 via PyPI
`requires_dist`, 2026-07-19), while every other LLM/embedder client
(`anthropic`, `google-genai`, `groq`, `voyageai`, `sentence-transformers`)
ships as an optional extra.

Deployments that pin a non-default LLM client still install the `openai` SDK
they never import at runtime, and organizations whose dependency policies
exclude specific vendors — or whose telemetry policies exclude phone-home
SDKs — currently cannot adopt `graphiti-core` at all, even though the client
layer is fully pluggable.

Proposed change, mirroring the existing extras pattern:

1. Move `openai` from base dependencies to a new default-documented extra
   (`graphiti-core[openai]`), keeping it in the quickstart so the default
   experience is one install flag away.
2. Guard the `openai` imports in the default client/embedder/reranker modules
   with the same lazy-import + helpful-error pattern used by the optional
   clients, so `import graphiti_core` succeeds without the SDK installed and
   only constructing an OpenAI-backed client requires it.
3. Move `posthog` to an extra (`graphiti-core[telemetry]`) or make telemetry
   import-guarded and default-off when the SDK is absent.

Happy to split 1+2 from 3 if telemetry is wanted as a separate discussion.

## Notes for the filing

- Verify against the current `main` `pyproject.toml` before opening (the
  0.29.2 metadata above is the evidence baseline; line numbers will drift).
- If maintainers accept, Holmes still keeps the owned subset (D-12); this PR
  is ecosystem hygiene, not a Holmes dependency plan.
