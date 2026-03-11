# Slack API Scopes Reference

Reference for `slack-cli apps create --scopes` and LLM/agent usage. Use with `--scopes "scope1,scope2,scope3"`.

**Workspace compatibility:**
- **Free** – Works on free Slack workspaces
- **Paid** – Requires Pro/Business+ (e.g. Slack Connect, Canvases, Workflow Builder)
- **Enterprise** – Enterprise Grid only (admin APIs, audit logs, org-wide search)

---

## Free Workspace Scopes

Core scopes that work on free workspaces.

| Scope | Description | Token |
|-------|-------------|-------|
| `channels:read` | View basic info about public channels | user, bot |
| `channels:write` | Manage user's public channels, create new ones | user |
| `channels:history` | View messages in public channels app is in | bot |
| `channels:join` | Join public channels | bot |
| `channels:manage` | Manage public channels app is in | bot |
| `channels:write.invites` | Invite members to public channels | user |
| `channels:write.topic` | Set description of public channels | user |
| `chat:write` | Send messages | user, bot |
| `chat:write.customize` | Send messages with custom username/avatar | bot |
| `chat:write.public` | Send messages to channels app isn't in | bot |
| `groups:read` | View basic info about private channels | user, bot |
| `groups:write` | Manage private channels | user, bot |
| `groups:history` | View messages in private channels | bot |
| `groups:write.invites` | Invite members to private channels | user |
| `groups:write.topic` | Set description of private channels | user |
| `im:read` | View basic info about DMs | user, bot |
| `im:history` | View messages in DMs | bot |
| `im:write` | Start direct messages | user, bot |
| `im:write.topic` | Set description in DMs | user |
| `mpim:read` | View basic info about group DMs | user, bot |
| `mpim:history` | View messages in group DMs | bot |
| `mpim:write` | Start group DMs | user, bot |
| `mpim:write.topic` | Set description in group DMs | user |
| `users:read` | View people in workspace | user, bot |
| `users:read.email` | View email addresses | user |
| `users:write` | Set presence | user |
| `users.profile:read` | View profile details | user |
| `users.profile:write` | Edit profile and status | user |
| `files:read` | View files in channels app is in | user, bot |
| `files:write` | Upload, edit, delete files | user, bot |
| `links:read` | View URLs in messages | bot |
| `links:write` | Show previews of URLs | user |
| `links.embed:write` | Embed video player URLs | bot |
| `reactions:read` | View emoji reactions | user, bot |
| `reactions:write` | Add and edit reactions | user, bot |
| `reminders:read` | View reminders | user |
| `reminders:write` | Add, remove, complete reminders | user |
| `pins:read` | View pinned content | user, bot |
| `pins:write` | Add and remove pins | user, bot |
| `stars:read` | View starred messages and files | user |
| `stars:write` | Add or remove stars | user |
| `dnd:read` | View Do Not Disturb settings | user |
| `dnd:write` | Edit Do Not Disturb settings | user |
| `team:read` | View workspace name, domain, icon | user, bot |
| `usergroups:read` | View user groups | user, bot |
| `usergroups:write` | Create and manage user groups | user |
| `bookmarks:read` | List bookmarks | user |
| `bookmarks:write` | Create, edit, remove bookmarks | user |
| `emoji:read` | View custom emoji | user, bot |
| `search:read` | Search workspace content | user |
| `search:read.files` | Search files | user |
| `search:read.im` | Search DMs | user |
| `search:read.mpim` | Search group DMs | user |
| `search:read.private` | Search private channels | user |
| `search:read.public` | Search public channels | user |
| `search:read.users` | Search users | user |
| `commands` | Add slash commands and shortcuts | bot |
| `incoming-webhook` | Post to specific channels | bot |
| `identify` | View user identity | user |
| `profile` | View avatar and workspace info | user |
| `email` | View user email | user |
| `openid` | View user identity (OpenID) | user |
| `tokens.basic` | Execute methods without extra scopes | user, bot |
| `calls:read` | View call info | user |
| `calls:write` | Start and manage calls | user |
| `remote_files:read` | View remote files | user |
| `remote_files:share` | Share remote files | user |
| `remote_files:write` | Add, edit, delete remote files | user |
| `metadata.message:read` | Read message metadata | bot |
| `app_mentions:read` | View messages mentioning app | bot |
| `authorizations:read` | List Events API authorizations | user |
| `connections:write` | Connect to Socket Mode | bot |
| `triggers:read` | Read Platform triggers | user, bot |
| `triggers:write` | Create Platform triggers | user, bot |
| `datastore:read` | View App Datastore | user, bot |
| `datastore:write` | Write to App Datastore | user, bot |
| `workflows.templates:read` | Read workflow templates | user |
| `workflows.templates:write` | Write workflow templates | user |
| `team.billing:read` | Read billing plan | user |
| `team.preferences:read` | Read workspace preferences | user |
| `apps.requests:write` | Create App Approval requests | user |
| `app_configurations:read` | Read app config via Manifest APIs | user |
| `app_configurations:write` | Write app config, create apps | user |
| `hosting:read` | View app info | user |
| `hosting:write` | Manage and deploy apps | user |

---

## Paid Workspace Scopes

Require Pro, Business+, or paid features (Slack Connect, Canvases, Lists).

| Scope | Description | Token | Notes |
|-------|-------------|-------|-------|
| `conversations.connect:read` | Receive Slack Connect invite events | bot | Slack Connect |
| `conversations.connect:write` | Create/accept Slack Connect invitations | bot | Slack Connect |
| `conversations.connect:manage` | Manage Slack Connect channels | bot | Slack Connect |
| `canvases:read` | Access canvas contents | user | Canvases |
| `canvases:write` | Create, edit, remove canvases | user | Canvases |
| `lists:read` | View Slack Lists | user | Lists |
| `lists:write` | Create, edit, delete Slack Lists | user | Lists |
| `assistant:write` | App as App Agent (AI) | bot | AI features |

---

## Enterprise Workspace Scopes

Enterprise Grid only. App must be installed by Org Owner at org level.

| Scope | Description | Token |
|-------|-------------|-------|
| `auditlogs:read` | View events from all workspaces in org | user |
| `search:read.enterprise` | Search content across enterprise | bot |
| `admin` | Administer workspace (access logs, SCIM) | user |
| `admin.analytics:read` | Access org analytics | user |
| `admin.app_activities:read` | View app execution logs | user |
| `admin.apps:read` | View apps and app requests | user |
| `admin.apps:write` | Manage apps | user |
| `admin.barriers:read` | Read information barriers | user |
| `admin.barriers:write` | Manage information barriers | user |
| `admin.chat:read` | Read messages in conversations | user |
| `admin.chat:write` | Delete, restore, update messages | user |
| `admin.conversations:manage_objects` | Manage channel details, external connections | user |
| `admin.conversations:read` | View channel members, topic, purpose | user |
| `admin.conversations:write` | Create and modify conversations | user |
| `admin.invites:read` | View invites and invite requests | user |
| `admin.invites:write` | Invite members, approve/deny requests | user |
| `admin.roles:read` | List role assignments | user |
| `admin.roles:write` | Add and remove role assignments | user |
| `admin.teams:read` | Access workspace info | user |
| `admin.teams:write` | Make changes to workspace | user |
| `admin.usergroups:read` | Access user groups | user |
| `admin.usergroups:write` | Manage user groups | user |
| `admin.users:read` | Access profile information | user |
| `admin.users:write` | Modify account information | user |
| `admin.workflows:read` | View all workflows | user |
| `admin.workflows:write` | Manage workflows | user |

---

## Legacy / Special Scopes

| Scope | Description | Token |
|-------|-------------|-------|
| `bot` | DM or mention app (legacy bot) | legacy bot |
| `client` | Receive all workspace events in real time | user |

---

## Default Scopes (slack-cli apps create)

When `--scopes` is omitted:

```
channels:read,channels:write,chat:write,users:read,files:read,files:write,groups:read,groups:write,links:write
```

---

## Example Commands

```bash
# Minimal (channels, chat, users)
slack-cli apps create --name "My CLI" --refresh-token xoxe-... --scopes "channels:read,chat:write,users:read"

# With search
slack-cli apps create --name "My CLI" --refresh-token xoxe-... --scopes "channels:read,chat:write,users:read,search:read"

# Full free-workspace set
slack-cli apps create --name "My CLI" --refresh-token xoxe-... --scopes "channels:read,channels:write,chat:write,users:read,files:read,files:write,groups:read,groups:write,links:write,reactions:read,reactions:write,reminders:read,reminders:write,pins:read,stars:read,dnd:read,dnd:write,team:read,usergroups:read,bookmarks:read,search:read"
```

---

**Source:** [Slack Scopes](https://api.slack.com/scopes) | [Slack Scope Reference](https://docs.slack.dev/reference/scopes)
