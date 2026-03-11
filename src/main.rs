mod client;
mod commands;
mod credentials;

use std::env;
use std::process::{self, Command};

const WEB_API_COMMANDS: &[&str] = &[
    "api", "apps", "auth", "bookmarks", "bots", "calls", "chat", "channels", "conversations",
    "dnd", "emoji", "files", "pins", "reactions", "reminders", "search", "stars", "team",
    "usergroups", "users", "workflows", "-h", "--help", "help",
];

fn find_slack_binary() -> String {
    if let Ok(p) = env::var("SLACK_CLI_PATH") {
        if !p.is_empty() {
            return p;
        }
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidates = [
                dir.join("slack"),
                dir.join("..").join("..").join("slack-cli").join("bin").join("slack"),
                dir.join("..").join("slack-cli").join("bin").join("slack"),
            ];
            for c in &candidates {
                if c.exists() {
                    return c.to_string_lossy().to_string();
                }
            }
        }
    }
    "slack".to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let first = args.get(1).map(|s| s.as_str());

    if let Some(arg) = first {
        if WEB_API_COMMANDS.contains(&arg) {
            let cli = commands::build_cli();
            let matches = cli.get_matches_from(args);
            let team_id = matches.get_one::<String>("team").map(|s| s.as_str());
            if let Err(e) = commands::run(&matches, team_id) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            return;
        }
    }

    let slack = find_slack_binary();
    let mut cmd = Command::new(&slack);
    if args.len() <= 1 {
        cmd.arg("--help");
    } else {
        cmd.args(&args[1..]);
    }
    cmd.stdin(process::Stdio::inherit());
    cmd.stdout(process::Stdio::inherit());
    cmd.stderr(process::Stdio::inherit());
    cmd.env_clear();
    cmd.envs(env::vars());
    match cmd.status() {
        Ok(status) => process::exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("Failed to run {}: {}", slack, e);
            process::exit(1);
        }
    }
}
