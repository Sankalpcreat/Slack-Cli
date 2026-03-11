use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("reactions")
        .about("Reactions")
        .subcommand(
            Command::new("add")
                .about("Add reaction")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<timestamp>))
                .arg(clap::arg!(<name> "emoji name e.g. thumbsup")),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove reaction")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<timestamp>))
                .arg(clap::arg!(<name>)),
        )
        .subcommand(
            Command::new("get")
                .about("Get reactions for message")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<timestamp>)),
        )
        .subcommand(
            Command::new("list")
                .about("List reactions")
                .arg(clap::arg!([user]))
                .arg(clap::arg!(--limit <N>).default_value("100"))
                .arg(clap::arg!(--cursor <C>)),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    let mut params = HashMap::new();
    let method = match name {
        "add" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("timestamp".into(), sub.get_one::<String>("timestamp").unwrap().clone());
            params.insert("name".into(), sub.get_one::<String>("name").unwrap().clone());
            "reactions.add"
        }
        "remove" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("timestamp".into(), sub.get_one::<String>("timestamp").unwrap().clone());
            params.insert("name".into(), sub.get_one::<String>("name").unwrap().clone());
            "reactions.remove"
        }
        "get" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("timestamp".into(), sub.get_one::<String>("timestamp").unwrap().clone());
            "reactions.get"
        }
        "list" => {
            params.insert("limit".into(), sub.get_one::<String>("limit").cloned().unwrap_or_else(|| "100".into()));
            if let Some(u) = sub.get_one::<String>("user") {
                params.insert("user".into(), u.clone());
            }
            if let Some(c) = sub.get_one::<String>("cursor") {
                params.insert("cursor".into(), c.clone());
            }
            "reactions.list"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
