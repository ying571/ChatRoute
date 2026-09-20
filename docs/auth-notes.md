# Auth Notes

This document records the current Codex App auth boundary for ChatRoute.

## Current Decision

ChatRoute writes Codex App `auth.json` in a local ChatGPT-shaped token mode:

```json
{
  "auth_mode": "chatgptAuthTokens",
  "OPENAI_API_KEY": null,
  "tokens": {
    "id_token": "<local ChatGPT-shaped JWT>",
    "access_token": "<local ChatGPT-shaped JWT>",
    "refresh_token": "",
    "account_id": "acct_chatroute_local"
  },
  "last_refresh": "2026-06-29T00:00:00Z"
}
```

Do not switch normal initialization to API-key-only auth. Pure API key mode can show plugins in newer Codex App builds, but upstream remote-control rejects it before ChatRoute can bridge the session.

## Config Injection

`chatroute configure-codex-app` writes:

- `chatgpt_base_url = "http://127.0.0.1:3847/backend-api"` for local backend fallback endpoints.
- A default `ai-gateway` provider at `http://127.0.0.1:3847/ai-gateway/v1`.
- `experimental_bearer_token = "dummy-token"` so model requests still authenticate to ChatRoute through the provider.
- A local `openai-curated` marketplace entry when the cached curated catalog exists.
- Cleanup for old plugin-blocking flags such as `apps = false`, `plugins = false`, `computer_use = false`.
- Cleanup for old ChatRoute-generated bundled remote plugin state.

ChatRoute does not publish `openai-bundled` plugins through remote `list` or `installed` fallback. Bundled plugins, including `computer-use`, must come from Codex App's own local `openai-bundled` marketplace.

## Legacy Compatibility

One in-progress ChatRoute build briefly wrote `OPENAI_API_KEY = "chatroute-dummy-key"` without `auth_mode`. Current code treats that as legacy ChatRoute-managed auth only so uninstall/cleanup can remove it safely. It is not the target auth shape.

The local `/backend-api/ps/plugins/*` fallback remains narrow:

- It serves cached `openai-curated` remote catalog/detail data.
- It provides read-only detail/skill fallback for old bundled remote IDs already stuck in UI/cache.
- It must not reintroduce bundled plugins into remote list/installed responses.
