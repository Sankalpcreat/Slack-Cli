#!/bin/bash
set -e

REPO="${SLACK_CLI_REPO:-https://github.com/Sankalpcreat/Slack-Cli}"
INSTALL_DIR="${SLACK_CLI_INSTALL_DIR:-/usr/local/bin}"
BIN_NAME="slack-cli"

echo "Installing slack-cli..."

if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust required. Install from https://rustup.rs"
    exit 1
fi

tmp=$(mktemp -d)
trap "rm -rf $tmp" EXIT
git clone --depth 1 "$REPO" "$tmp"
(cd "$tmp" && cargo build --release)
sudo cp "$tmp/target/release/slack-cli" "$INSTALL_DIR/$BIN_NAME"

echo ""
echo "Installed: $INSTALL_DIR/$BIN_NAME"
echo ""
echo "1. Get token: api.slack.com/apps → Your App → OAuth & Permissions → User OAuth Token"
echo "2. Login: $BIN_NAME auth login --token xoxp-YOUR-TOKEN"
echo "3. Cursor/agents: export SLACK_TOKEN=xoxp-YOUR-TOKEN"
