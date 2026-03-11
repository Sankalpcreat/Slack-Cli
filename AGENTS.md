# Agent Guide — slack-cli

**Read this file first.** This is the primary onboarding document for LLMs and agents when cloning or working with this repository.

---

## What This Project Is

**slack-cli** — A Rust CLI for the Slack Web API. 107 commands for chat, channels, files, users, search, reactions, and more. Use Slack from the terminal, scripts, or AI agents (Cursor, Claude Code, Codex) without opening the Slack app.

**Not an official Slack project.** Independently built using the public Slack API.

---

## Quick Start for Agents

1. **Install:** `cargo build --release` (requires Rust 1.85+)
2. **Auth:** User must run `slack-cli auth login --token xoxp-...` or set `SLACK_TOKEN` in the environment
3. **Run:** `slack-cli <command> <args>`

For agents: set `SLACK_TOKEN=xoxp-your-token` in the environment. The CLI reads it before the credentials file.

---

## Repository Map — Where to Look

| Path | Purpose |
|------|---------|
| **AGENTS.md** | This file. Start here. |
| **README.md** | User-facing docs: installation, auth flows, command reference |
| **docs/SCOPES.md** | All Slack scopes by free / paid / enterprise workspace |
| **Cargo.toml** | Rust package: name `cli-slack` (crates.io), binary `slack-cli`, deps (clap, reqwest, serde, anyhow) |
| **install.sh** | One-liner install: clones repo, builds, copies to `/usr/local/bin` |
| **scripts/scope-setup.sh** | Add scopes to existing app (needs `SLACK_CONFIG_TOKEN`, `SLACK_APP_ID`) |
| **src/main.rs** | Entry point: routes to Web API CLI or falls back to `slack` binary |
| **src/client.rs** | HTTP client: `call()`, `call_opt()`, `upload_file()`, `download_file()` |
| **src/credentials.rs** | Token load/save: `~/.slack/credentials.json`, env `SLACK_TOKEN` override |
| **src/commands/mod.rs** | CLI builder and command router |
| **src/commands/*.rs** | One file per command group (api, auth, apps, chat, files, etc.) |

---

## File Structure

```
slack-cli/
├── AGENTS.md           ← You are here
├── README.md
├── Cargo.toml
├── install.sh
├── .gitignore
├── assets/
│   └── slack-cli-banner.png
├── docs/
│   └── SCOPES.md       # Scope reference (free/paid/enterprise)
├── scripts/
│   └── scope-setup.sh  # Add scopes to existing app
└── src/
    ├── main.rs
    ├── client.rs
    ├── credentials.rs
    └── commands/
        ├── mod.rs      # Router
        ├── api.rs
        ├── apps.rs     # create, token rotate, manifest, scopes
        ├── auth.rs
        ├── bookmarks.rs
        ├── bots.rs
        ├── calls.rs
        ├── chat.rs
        ├── conversations.rs
        ├── dnd.rs
        ├── emoji.rs
        ├── files.rs
        ├── pins.rs
        ├── reactions.rs
        ├── reminders.rs
        ├── search.rs
        ├── stars.rs
        ├── team.rs
        ├── usergroups.rs
        ├── users.rs
        └── workflows.rs
```

---

## Installation (for Users / Agents to Run)

**From source (after clone):**
```bash
git clone https://github.com/Sankalpcreat/Slack-Cli.git
cd Slack-Cli
cargo build --release
# Binary: target/release/slack-cli
# Optional: sudo cp target/release/slack-cli /usr/local/bin/
```

**One-liner:**
```bash
curl -fsSL https://raw.githubusercontent.com/Sankalpcreat/Slack-Cli/main/install.sh | sh
```

**Requirements:** Rust 1.85+ ([rustup.rs](https://rustup.rs))

---

## Authentication — Two Flows

### Flow A: CLI Creates App
1. Get refresh token from api.slack.com/apps → Your App Configuration Tokens → Generate Token
2. `slack-cli apps create --name "My CLI" --refresh-token xoxe-...`
3. Add redirect URL `https://localhost` in Slack UI, Install, copy token
4. `slack-cli auth login --token xoxp-...`

### Flow B: Manual
1. Create app at [api.slack.com/apps](https://api.slack.com/apps)
2. Add scopes, Install to Workspace
3. Copy User OAuth Token (`xoxp-...`)
4. `slack-cli auth login --token xoxp-...`

**Scopes:** See `docs/SCOPES.md`. Override with `--scopes "scope1,scope2"` on `apps create`.

---

## Environment Variables

| Variable | Use |
|----------|-----|
| `SLACK_TOKEN` | Token override. Use for agents, CI, Docker. Takes precedence over credentials file. |
| `SLACK_CONFIG_TOKEN` | Config token (xoxe-) for `apps manifest` / `apps scopes` |
| `SLACK_REFRESH_TOKEN` | Refresh token for `apps create` |
| `SLACK_CLI_REPO` | Repo URL for install script (default: GitHub) |
| `SLACK_CLI_INSTALL_DIR` | Install path (default: `/usr/local/bin`) |
| `SLACK_CLI_PATH` | Override path to `slack` binary (used when delegating to Slack CLI) |

---

## Key Commands for Agents

| Command | Use |
|---------|-----|
| `slack-cli auth test` | Verify token works |
| `slack-cli auth login --token xoxp-...` | Save token |
| `slack-cli apps create --name N --refresh-token T` | Create Slack app |
| `slack-cli api <method> [-p k=v ...]` | Raw API call |
| `slack-cli chat post <channel> "text"` | Post message |
| `slack-cli conversations list` | List channels |
| `slack-cli files upload <channel> <path>` | Upload file |
| `slack-cli users list` | List users |
| `slack-cli search all -q "query"` | Search |

**Global:** `-t, --team <ID>` — Use specific workspace (default: first in credentials)

---

## Credentials and Token Resolution

1. **SLACK_TOKEN** env var — used if set and non-empty
2. **~/.slack/credentials.json** — keyed by team_id or `"default"`
3. Error if neither has a valid token

Credentials file structure: `{ "team_id": { "token", "team_id", "team_domain", "user_id" } }`

---

## Code Architecture

- **main.rs:** If first arg is a Web API command (api, auth, chat, etc.), run the Rust CLI. Otherwise delegate to `slack` binary (Slack’s official CLI).
- **client.rs:** All Slack API calls go through `client::call(token, method, params)`. Uses `reqwest` blocking, Bearer auth, form-encoded POST. `call_opt(None, ...)` for token-less calls (e.g. `tooling.tokens.rotate`).
- **credentials.rs:** Load token from env or file. Save on `auth login`.
- **commands/mod.rs:** Builds clap CLI, routes to `*::run()` per subcommand. `auth login` bypasses token load (uses `--token` arg).
- **commands/*.rs:** Each file defines `command()` and `run()`. Uses `client::call()` for API.

---

## Scripts

- **install.sh:** Clones repo, `cargo build --release`, copies to `$INSTALL_DIR`
- **scripts/scope-setup.sh:** Adds scopes to existing app via `apps scopes add`, opens browser. Needs `SLACK_CONFIG_TOKEN`, `SLACK_APP_ID`.

---

## .gitignore

`/target/`, `.slack/`, `credentials.json`, `*.log`, `.env`, `*.pem`, `*.key`

---

## For LLM / Agent Integration

1. Set `SLACK_TOKEN=xoxp-your-token` in environment or agent config
2. Ensure `slack-cli` is on PATH (or use full path to binary)
3. Run commands: `slack-cli chat post C123 "Hello"`

**Cursor:** Settings → Cursor Settings → Features → Environment → add `SLACK_TOKEN`

---

## Next Steps After Reading This

- **Install:** See [Installation](#installation-for-users--agents-to-run)
- **Auth:** See [Authentication](#authentication--two-flows)
- **Commands:** See README.md [Commands Reference](README.md#commands-reference)
- **Scopes:** See docs/SCOPES.md
