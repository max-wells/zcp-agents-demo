# Zerops MCP + zcli, quick start

Machine bound to Zerops project via ZCP (MCP server `zerops`). Claude Code talks to Zerops through `zerops_*` tools, not raw `zcli`.

## First run

1. Just open Claude Code normally from `demo-app/`: `claude`.
2. `zerops_*` tools appear automatically (MCP server already configured in project).

`.mcp.json` points `zerops` at `./zcp-serve.sh`, a wrapper that sources `.env` itself before exec'ing `zcp serve`. No need to source `.env` into the parent shell anymore — `ZCP_API_KEY` never has to live in Claude Code's own process env.

If `/mcp` shows `zerops` as `CONNECTION_CLOSED`: check `.env` has `ZCP_API_KEY` set, `zcp-serve.sh` is executable (`chmod +x zcp-serve.sh`), then `/mcp` reconnect.

## Core tools (19 total)

1. `zerops_workflow` — orchestrator. `action="start" workflow="develop"` (or `bootstrap` for new/adopted services) opens a session, guides discover → provision → close.
2. `zerops_deploy targetService="app"` — pushes working dir, builds, deploys. Needs `zerops.yaml` at repo root + `zcli` installed locally.
3. `zerops_discover` — live state: services, status, adoption.
4. `zerops_env` — read/set env vars (get/set/delete/generate-dotenv).
5. `zerops_logs` — debug runtime issues from service logs.
6. `zerops_events` — debug build/deploy issues from platform events.
7. `zerops_verify` — post-deploy healthcheck.
8. `zerops_process` — check/wait/cancel async work (build, deploy, queued ops).
9. `zerops_manage` — service lifecycle: start/stop/restart/reload/connect-storage.
10. `zerops_scale` — resource scaling (CPU/RAM/disk).
11. `zerops_subdomain` — public URL toggle.
12. `zerops_import` — apply an `import.yaml`.
13. `zerops_export` — export project/service definition.
14. `zerops_delete` — remove a service.
15. `zerops_mount` — local-storage mount ops.
16. `zerops_knowledge` — Zerops docs/playbooks lookup.
17. `zerops_preprocess` — internal workflow plumbing.
18. `zerops_workspace_manifest` — internal workflow plumbing (manifest building).
19. `zerops_record_fact` — internal workflow plumbing (fact recording for session state).

## What ZCP via MCP gives you that reading code alone doesn't

| # | Capability | Detail |
|---|---|---|
| 1 | Real DB schema | Local migrations show history, but there's no guarantee that matches what's running in prod. `zerops_env`/logs can reveal constraints added by hand, or a migration never applied. A `CHECK(quantity>0)` like this is often added directly in prod SQL, never committed as a migration. |
| 2 | Real error logs | `zerops_logs`/`zerops_verify` show the actual crash (500 on which exact request, exact Postgres error message), not a guess from reading handler code. |
| 3 | Test against the live instance | After a fix, redeploy + a real request against the running server confirms 500→400 for real. Reading code never proves a fix works, only that it compiles. |
| 4 | Runtime state | Process up/down, env vars actually resolved (`${db_connectionString}` only resolves inside the container, invisible statically), healthcheck status. |
| 5 | Real deploy/rollback | Code review proposes a diff, but nothing actually happens; here the agent pushes the change to prod, effect visible immediately. |

Pitch line: agents need real environments, not just code generation.
