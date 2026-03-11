#!/bin/bash
set -e

CLI="${CLI:-./target/release/slack-cli}"
CONFIG_TOKEN="${SLACK_CONFIG_TOKEN:?Set SLACK_CONFIG_TOKEN (xoxe-...)}"
APP_ID="${SLACK_APP_ID:?Set SLACK_APP_ID (e.g. A0AKRN7V4NR - find in api.slack.com/apps/APP_ID/...)}"

USER_SCOPES="channels:read,groups:read,channels:manage,groups:write,im:read,mpim:read,emoji:read,users:read.email,users.profile:read,users:write,reactions:read,reminders:read,pins:read,stars:read,dnd:read,dnd:write,team:read,usergroups:read,usergroups:write,bookmarks:read,bookmarks:write,calls:read,calls:write"

echo "=== Step 1: Adding scopes to app $APP_ID ==="
$CLI apps --config-token "$CONFIG_TOKEN" scopes add "$APP_ID" --user "$USER_SCOPES"
echo "Scopes added."

echo ""
echo "=== Step 2: Opening Install page in browser ==="
INSTALL_URL="https://api.slack.com/apps/${APP_ID}/oauth"
echo "Open: $INSTALL_URL"
echo "Click 'Install to Workspace' (or 'Reinstall') and approve."
echo ""

if command -v open >/dev/null 2>&1; then
    open "$INSTALL_URL"
elif command -v xdg-open >/dev/null 2>&1; then
    xdg-open "$INSTALL_URL"
else
    echo "Open this URL manually: $INSTALL_URL"
fi

echo ""
echo "=== Step 3: After you approve ==="
echo "1. On the same page, under 'OAuth Tokens for Your Workspace'"
echo "2. Copy the 'User OAuth Token' (starts with xoxp-)"
echo "3. Run:"
echo "   $CLI auth login --token xoxp-YOUR-NEW-TOKEN"
echo ""
echo "Credentials saved to ~/.slack/credentials.json"
