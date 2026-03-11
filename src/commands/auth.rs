use crate::client;
use crate::credentials;
use anyhow::{Context, Result};
use clap::Command;
use serde_json::Value;

pub fn command() -> Command {
    Command::new("auth")
        .about("Auth")
        .subcommand(Command::new("test").about("Test authentication"))
        .subcommand(Command::new("revoke").about("Revoke current token"))
        .subcommand(
            Command::new("login")
                .about("Save token to credentials (re-auth after revoke)")
                .arg(clap::arg!(--token <T> "User OAuth token (xoxp-...)").required(true)),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    match m.subcommand() {
        Some(("test", _)) => {
            let out = client::call(token, "auth.test", None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("revoke", _)) => {
            let out = client::call(token, "auth.revoke", None)?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("login", sm)) => {
            let new_token = sm.get_one::<String>("token").context("--token required")?;
            let out = client::call(new_token, "auth.test", None)?;
            let dec: Value = serde_json::from_slice(&out).context("parse auth.test")?;
            if !dec.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
                anyhow::bail!("invalid token: {}", dec.get("error").unwrap_or(&Value::Null));
            }
            credentials::save(
                new_token,
                dec.get("team_id").and_then(|v| v.as_str()).unwrap_or(""),
                dec.get("team").and_then(|v| v.as_str()).unwrap_or(""),
                dec.get("user_id").and_then(|v| v.as_str()).unwrap_or(""),
            )?;
            println!("Token saved. Team: {}", dec.get("team").unwrap_or(&Value::Null));
        }
        _ => {}
    }
    Ok(())
}
