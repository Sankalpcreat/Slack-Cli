use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("team")
        .about("Team info")
        .subcommand(Command::new("info").about("Get team info"))
        .subcommand(
            Command::new("access-logs")
                .about("Get access logs")
                .arg(clap::arg!(--count <N>).default_value("100"))
                .arg(clap::arg!(--page <N>).default_value("1")),
        )
        .subcommand(Command::new("billable").about("Get billable info").arg(clap::arg!([user])))
        .subcommand(
            Command::new("integration-logs")
                .about("Get integration logs")
                .arg(clap::arg!(--count <N>).default_value("100"))
                .arg(clap::arg!(--page <N>).default_value("1")),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    let mut params = HashMap::new();
    let method = match name {
        "info" => {
            let out = client::call(token, "team.info", None)?;
            println!("{}", String::from_utf8_lossy(&out));
            return Ok(());
        }
        "access-logs" => {
            params.insert("count".into(), sub.get_one::<String>("count").cloned().unwrap_or_else(|| "100".into()));
            params.insert("page".into(), sub.get_one::<String>("page").cloned().unwrap_or_else(|| "1".into()));
            "team.accessLogs"
        }
        "billable" => {
            if let Some(u) = sub.get_one::<String>("user") {
                params.insert("user".into(), u.clone());
            }
            "team.billableInfo"
        }
        "integration-logs" => {
            params.insert("count".into(), sub.get_one::<String>("count").cloned().unwrap_or_else(|| "100".into()));
            params.insert("page".into(), sub.get_one::<String>("page").cloned().unwrap_or_else(|| "1".into()));
            "team.integrationLogs"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
