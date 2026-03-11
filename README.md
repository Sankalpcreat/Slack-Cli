# slack-cli

<p align="center"><img src="assets/slack-cli-banner.png" width="400" alt="Slack CLI" /></p>

> **Disclaimer:** This is not an official Slack project. Independently built as a personal side project using the public [Slack API](https://api.slack.com/), and open sourced for the community. Not affiliated with or endorsed by Slack. Slack and the Slack logo are trademarks of Slack Technologies, LLC.

Slack Web API CLI. 107 commands for chat, channels, files, users, search, reactions, and more. Use Slack from the terminal, scripts, or AI agents (Cursor, Claude Code, Codex) without opening the Slack app.

---

## Table of Contents

- [Installation](#installation)
- [Authentication](#authentication)
- [Repository Structure](#repository-structure)
- [Commands Reference](#commands-reference)
  - **Commands:** [api](#api) | [auth](#auth) | [chat](#chat) | [conversations](#conversations-channels) | [files](#files) | [users](#users) | [search](#search) | [reactions](#reactions) | [reminders](#reminders) | [pins](#pins) | [stars](#stars) | [dnd](#dnd-do-not-disturb) | [team](#team) | [usergroups](#usergroups) | [bookmarks](#bookmarks) | [emoji](#emoji) | [bots](#bots) | [workflows](#workflows) | [calls](#calls) | [apps](#apps)
- [Scripts](#scripts)
- [Environment Variables](#environment-variables)
- [LLM / Agent Integration](#llm--agent-integration)
- [Requirements](#requirements)
- [License](#license)

---

## Installation

### One-liner (requires Rust)

```bash
curl -fsSL https://raw.githubusercontent.com/Sankalpcreat/Slack-Cli/main/install.sh | sh
```

### From source

```bash
git clone https://github.com/Sankalpcreat/Slack-Cli.git && cd Slack-Cli
cargo build --release
sudo cp target/release/slack-cli /usr/local/bin/
```

### Cargo install (after publishing to crates.io)

```bash
cargo install slack-cli
```

**Requirements:** Rust 1.85+ ([rustup.rs](https://rustup.rs))

---

## Authentication

Two ways to get a token: **create** (CLI creates the app for you) or **manual** (step-by-step in the UI).

---

### Flow A: Create app (CLI creates it)

The CLI creates the app for you. You only need a **refresh token** (one-time setup) and to add the redirect URL in the Slack UI.

1. **Get refresh token (one-time):** [api.slack.com/apps](https://api.slack.com/apps) → scroll below app list → **Your App Configuration Tokens** → **Generate Token**. Copy the **refresh token** (config token expires in 12h; refresh token is for future runs).
2. **Create app:**

```bash
slack-cli apps create --name "My CLI" --refresh-token xoxe-your-refresh-token
```

3. **Add redirect URL:** Browser opens automatically → **OAuth & Permissions** → **Redirect URLs** → add `https://localhost`
4. **Install:** Click **Install to Workspace** → approve → copy **User OAuth Token** (`xoxp-...`)
5. **Login:**

```bash
slack-cli auth login --token xoxp-YOUR-TOKEN
```

**Scopes:** Default is a limited set: `channels:read`, `channels:write`, `chat:write`, `users:read`, `files:read`, `files:write`, `groups:read`, `groups:write`, `links:write`. Override with `--scopes`:

```bash
slack-cli apps create --name "My CLI" --refresh-token xoxe-... --scopes "channels:read,chat:write,users:read,search:read"
```

Full scope reference: [docs/SCOPES.md](docs/SCOPES.md) (free / paid / enterprise).

---

### Flow B: Manual (step-by-step)

1. **Create app:** [api.slack.com/apps](https://api.slack.com/apps) → **Create New App** → **From scratch** → name it, select workspace
2. **Add scopes:** **OAuth & Permissions** → **User Token Scopes** → add what you need (see table below)
3. **Install:** **Install to Workspace** → approve
4. **Copy token:** User OAuth Token (`xoxp-...`) on the same page
5. **Login:**

```bash
slack-cli auth login --token xoxp-YOUR-TOKEN
```

No redirect URL. Token is copied from the web UI.

| Scope | Use |
|-------|-----|
| `channels:read`, `channels:write` | List, join, post to channels |
| `chat:write` | Post messages |
| `users:read` | List users |
| `files:read`, `files:write` | Upload, download files |
| `groups:read`, `groups:write` | Private channels |
| `links:write` | Custom link unfurling |

See [docs/SCOPES.md](docs/SCOPES.md) for full list (free / paid / enterprise).

---

## Repository Structure

**For LLMs/agents:** Read [AGENTS.md](AGENTS.md) first — repo map, install, auth, commands.

**Scopes:** [docs/SCOPES.md](docs/SCOPES.md) — all scopes by free / paid / enterprise workspace.

```
slack-cli/
├── AGENTS.md           # Agent/LLM onboarding (read first)
├── docs/
│   └── SCOPES.md       # Scope reference (free/paid/enterprise)
├── Cargo.toml          # Rust package manifest
├── src/
│   ├── main.rs         # Entry point
│   ├── client.rs       # HTTP client for Slack API
│   ├── credentials.rs  # Token load/save
│   └── commands/       # Command implementations
│       ├── api.rs
│       ├── auth.rs
│       ├── chat.rs
│       ├── conversations.rs
│       ├── files.rs
│       ├── users.rs
│       └── ...
├── scripts/
│   └── scope-setup.sh  # Add scopes to existing app (advanced)
├── install.sh          # Install script
├── slack-cli-banner.png
└── README.md
```

---

## Commands Reference

**Global option:** `-t, --team <ID>` — Use specific workspace (default: first in credentials)

---

### api

Call any Slack Web API method directly.

| Command | Use |
|---------|-----|
| `slack-cli api <method> [-p key=value ...]` | Call `method` with optional params |

**Example:**
```bash
slack-cli api auth.test
slack-cli api chat.postMessage -p channel=C123 -p text="Hello"
```

---

### auth

| Command | Use | When |
|---------|-----|------|
| `auth test` | Verify token works | After login, debugging |
| `auth login --token xoxp-...` | Save token to credentials | First setup, re-auth after revoke |
| `auth revoke` | Invalidate current token | Security, rotate tokens |

**Examples:**
```bash
slack-cli auth test
slack-cli auth login --token xoxp-123-456-789
slack-cli auth revoke
```

---

### chat

| Command | Use | When |
|---------|-----|------|
| `chat post <channel> <text>... [--thread TS]` | Post message | Send to channel or thread |
| `chat delete <channel> <ts>` | Delete message | Remove your message |
| `chat update <channel> <ts> <text>...` | Edit message | Fix typo, update content |
| `chat schedule <channel> <post_at_unix_ts> <text>...` | Schedule message | Post at future time |
| `chat scheduled [--channel ID]` | List scheduled messages | View pending posts |
| `chat delete-scheduled <channel> <scheduled_message_id>` | Cancel scheduled | Remove scheduled post |
| `chat permalink <channel> <ts>` | Get message URL | Share link to message |
| `chat ephemeral <channel> <user> <text>...` | Ephemeral message | Only visible to one user |
| `chat me <channel> <text>...` | /me message | Status-style message |
| `chat unfurl <channel> <ts> --unfurls <JSON>` | Custom link preview | App unfurling (needs `links:write`) |

**Examples:**
```bash
slack-cli chat post C123456 "Hello world"
slack-cli chat post C123456 "Reply" --thread 1234567890.123456
slack-cli chat delete C123456 1234567890.123456
slack-cli chat ephemeral C123456 U123456 "Only you see this"
```

---

### conversations (channels)

| Command | Use | When |
|---------|-----|------|
| `conversations list [--limit N] [--exclude-archived]` | List channels | Find channel IDs |
| `conversations history <channel> [--limit N] [--oldest TS] [--latest TS]` | Channel history | Read messages |
| `conversations replies <channel> <ts> [--limit N]` | Thread replies | Read thread |
| `conversations info <channel>` | Channel details | Get name, topic, members |
| `conversations create <name> [--private]` | Create channel | New public or private |
| `conversations archive <channel>` | Archive channel | Hide from list |
| `conversations unarchive <channel>` | Unarchive | Restore archived |
| `conversations invite <channel> <user_id>...` | Invite users | Add members |
| `conversations join <channel>` | Join channel | Join public channel |
| `conversations leave <channel>` | Leave channel | Leave channel |
| `conversations kick <channel> <user_id>` | Remove user | Moderation |
| `conversations rename <channel> <name>` | Rename channel | Change name |
| `conversations members <channel>` | List members | See who's in channel |
| `conversations open [user_id]` | Open DM/MPIM | Start DM |
| `conversations close <channel>` | Close DM/MPIM | Close DM (DM only) |
| `conversations set-topic <channel> <topic>...` | Set topic | Channel description |
| `conversations set-purpose <channel> <purpose>...` | Set purpose | Channel purpose |
| `conversations mark <channel> <ts>` | Mark read | Mark as read |
| `conversations accept-shared-invite <invite_id> <channel_name>` | Accept Slack Connect | Cross-workspace |
| `conversations approve-shared-invite <invite_id>` | Approve invite | Admin |
| `conversations decline-shared-invite <invite_id>` | Decline invite | Reject Slack Connect |
| `conversations invite-shared <channel> <emails>...` | Invite to Slack Connect | Cross-workspace |
| `conversations list-connect-invites` | List Connect invites | View pending |

**Examples:**
```bash
slack-cli conversations list --limit 20
slack-cli conversations create my-channel
slack-cli conversations invite C123 U456 U789
slack-cli conversations history C123 --limit 10
```

---

### files

| Command | Use | When |
|---------|-----|------|
| `files list [--limit N] [--channel ID] [--user ID]` | List files | Find file IDs |
| `files info <file_id>` | File details | Get metadata |
| `files upload <channel> <file_path> [--comment TEXT]` | Upload file | Share file |
| `files download <file_id> [output_path]` | Download file | Save to disk |
| `files delete <file_id>` | Delete file | Remove file |
| `files share-public <file_id>` | Make public | Get shareable URL (paid) |
| `files revoke-public <file_id>` | Revoke public URL | Remove public link |

**Examples:**
```bash
slack-cli files list --limit 10
slack-cli files upload C123 ./report.pdf --comment "Q4 report"
slack-cli files download F123 ./saved-file.pdf
```

---

### users

| Command | Use | When |
|---------|-----|------|
| `users list [--limit N]` | List users | Find user IDs |
| `users info <user_id>` | User details | Get profile |
| `users lookup <email>` | Lookup by email | Find user by email |
| `users presence <user_id>` | Presence status | Online/away |
| `users set-presence <auto|away>` | Set presence | Set your status |
| `users profile <user_id>` | Get profile | Full profile |
| `users profile-set <profile_json>` | Set profile | Update status, emoji |
| `users set-photo <image_path>` | Set avatar | Upload profile photo |
| `users delete-photo` | Remove avatar | Clear profile photo |
| `users conversations [--limit N] [--types T] [--user ID]` | User's channels | Channels user is in |

**Examples:**
```bash
slack-cli users list --limit 50
slack-cli users info U123456
slack-cli users profile-set '{"status_text":"In a meeting","status_emoji":":calendar:"}'
```

---

### search

| Command | Use | When |
|---------|-----|------|
| `search all -q <query> [--count N] [--sort S]` | Search all | Messages + files |
| `search messages -q <query> [--count N] [--sort S]` | Search messages | Messages only |
| `search files -q <query> [--count N] [--sort S]` | Search files | Files only |

**Examples:**
```bash
slack-cli search all -q "deployment"
slack-cli search messages -q "bug" --count 10
```

---

### reactions

| Command | Use | When |
|---------|-----|------|
| `reactions add <channel> <timestamp> <name>` | Add emoji | React to message |
| `reactions remove <channel> <timestamp> <name>` | Remove emoji | Remove reaction |
| `reactions get <channel> <timestamp>` | Get reactions | See who reacted |
| `reactions list [--limit N] [--user ID] [--cursor C]` | List reactions | Your or user's reactions |

**Example:**
```bash
slack-cli reactions add C123 1234567890.123456 thumbsup
```

---

### reminders

| Command | Use | When |
|---------|-----|------|
| `reminders list` | List reminders | View your reminders |
| `reminders add <text> <time>` | Add reminder | Create reminder |
| `reminders delete <reminder_id>` | Delete reminder | Remove reminder |
| `reminders complete <reminder_id>` | Complete | Mark done |
| `reminders info <reminder_id>` | Reminder details | Get info |

**Note:** Reminders API is deprecated. `info`, `complete`, `delete` may return `not_found`. Use Workflow Builder for new workflows.

---

### pins

| Command | Use | When |
|---------|-----|------|
| `pins list <channel>` | List pins | See pinned items |
| `pins add <channel> <timestamp> [--file ID]` | Pin message | Pin to channel |
| `pins remove <channel> <timestamp> [--file ID]` | Unpin | Remove pin |

---

### stars

| Command | Use | When |
|---------|-----|------|
| `stars add <channel> <timestamp> [--file ID]` | Star item | Save for later |
| `stars remove <channel> <timestamp> [--file ID]` | Unstar | Remove star |
| `stars list [--limit N] [--cursor C]` | List stars | Your starred items |

---

### dnd (Do Not Disturb)

| Command | Use | When |
|---------|-----|------|
| `dnd info` | DND status | Check if DND is on |
| `dnd snooze <num_minutes>` | Snooze notifications | Temporary quiet |
| `dnd end-snooze` | End snooze | Resume notifications |
| `dnd end` | End DND | Turn off DND |

---

### team

| Command | Use | When |
|---------|-----|------|
| `team info` | Team info | Workspace name, domain |
| `team access-logs [--count N] [--page N] [--before N]` | Access logs | Audit (paid) |
| `team billable [--user ID]` | Billable info | Billing (paid) |
| `team integration-logs [--count N] [--page N] [--service ID]` | Integration logs | App activity |

---

### usergroups

| Command | Use | When |
|---------|-----|------|
| `usergroups list [--include-count]` | List groups | Get group IDs |
| `usergroups create <name> [--description D] [--handle H]` | Create group | New user group (paid) |
| `usergroups update <usergroup_id> [--name N] [--description D] [--handle H]` | Update group | Change name, etc. |
| `usergroups disable <usergroup_id>` | Disable group | Deactivate |
| `usergroups enable <usergroup_id>` | Enable group | Reactivate |
| `usergroups users <usergroup_id>` | List group users | Members |
| `usergroups users-update <usergroup_id> <user_ids>...` | Update members | Set group members |

---

### bookmarks

| Command | Use | When |
|---------|-----|------|
| `bookmarks list <channel>` | List bookmarks | Channel bookmarks |
| `bookmarks add <channel_id> <title> <link> [--type link]` | Add bookmark | Save link |
| `bookmarks edit <channel_id> <bookmark_id> <title> <link>` | Edit bookmark | Update |
| `bookmarks remove <channel_id> <bookmark_id>` | Remove bookmark | Delete |

---

### emoji

| Command | Use | When |
|---------|-----|------|
| `emoji list [--include-categories]` | List emoji | Workspace emoji |

---

### bots

| Command | Use | When |
|---------|-----|------|
| `bots info <bot_id>` | Bot info | Get bot details |

---

### workflows

| Command | Use | When |
|---------|-----|------|
| `workflows step-completed <workflow_step_execute_id> [outputs_json]` | Complete step | Workflow Builder callback |
| `workflows step-failed <workflow_step_execute_id> <error>...` | Fail step | Workflow Builder error |

**Note:** Requires Workflow Builder app (paid plan).

---

### calls

| Command | Use | When |
|---------|-----|------|
| `calls add <external_unique_id> <join_url> [--title T] [--desktop-join-url U]` | Register call | Start call |
| `calls end <call_id>` | End call | End call |
| `calls info <call_id>` | Call info | Get call details |
| `calls update <call_id> [--title T] [--join-url U]` | Update call | Change title/URL |
| `calls participants-add <call_id> <user_ids>...` | Add participants | Add users to call |
| `calls participants-remove <call_id> <user_ids>...` | Remove participants | Remove from call |

---

### apps

**Requires:** `SLACK_CONFIG_TOKEN` (xoxe-) or `--config-token`

| Command | Use | When |
|---------|-----|------|
| `apps create --name N [--refresh-token T \| --config-token T] [--scopes S]` | Create app from manifest | Flow A |
| `apps token rotate --refresh-token T` | Get new config token | Token expired |
| `apps manifest export <app_id>` | Export manifest | Get app config |
| `apps manifest update <app_id> <manifest_file>` | Update manifest | Update app |
| `apps scopes add <app_id> [--user S] [--bot S]` | Add scopes | Add to app |
| `apps scopes remove <app_id> [--user S] [--bot S]` | Remove scopes | Remove from app |
| `apps permissions info` | Permissions info | Get scopes (deprecated) |
| `apps permissions request <scopes> [trigger_id]` | Request scopes | Request additional |
| `apps permissions resources [resource_types] [cursor]` | List resources | Permission resources |

---

## Scripts

### install.sh

One-command install. Clones repo, builds, copies to `/usr/local/bin`.

```bash
curl -fsSL https://raw.githubusercontent.com/Sankalpcreat/Slack-Cli/main/install.sh | sh
```

**Env vars:** `SLACK_CLI_REPO`, `SLACK_CLI_INSTALL_DIR`

---

## Environment Variables

| Variable | Use |
|----------|-----|
| `SLACK_TOKEN` | Token (overrides credentials file). Use for agents, CI, Docker. |
| `SLACK_CONFIG_TOKEN` | Config token (xoxe-) for `apps manifest` / `apps scopes` |
| `SLACK_REFRESH_TOKEN` | Refresh token for `apps create` |
| `SLACK_CLI_REPO` | Repo URL for install script |
| `SLACK_CLI_INSTALL_DIR` | Install path for install script |

---

## LLM / Agent Integration

**Start here:** [AGENTS.md](AGENTS.md) — full repo map, install, auth, and command reference for agents.

### Cursor / Claude Code / Codex

1. Set `SLACK_TOKEN` in environment or agent config
2. Ensure `slack-cli` is on PATH
3. Agent can run commands: `slack-cli chat post C123 "Hello"`

### Add to Cursor

**Settings → Cursor Settings → Features → Environment:**

```
SLACK_TOKEN=xoxp-your-token
```

### Shell profile

```bash
export SLACK_TOKEN=xoxp-your-token
```

### Inline

```bash
SLACK_TOKEN=xoxp-xxx slack-cli conversations list
```

---

## Requirements

- **Rust:** 1.85+ (for edition 2024)
- **Slack app:** Create at api.slack.com, install to workspace
- **Token:** User OAuth Token (xoxp-) with required scopes

---

## License

MIT
