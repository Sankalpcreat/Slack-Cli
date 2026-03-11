use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("stars")
        .about("Stars")
        .subcommand(
            Command::new("add")
                .about("Star item")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<timestamp>))
                .arg(clap::arg!(--file <ID>)),
        )
        .subcommand(
            Command::new("remove")
                .about("Unstar item")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<timestamp>))
                .arg(clap::arg!(--file <ID>)),
        )
        .subcommand(
            Command::new("list")
                .about("List starred items")
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
            if let Some(f) = sub.get_one::<String>("file") {
                params.insert("file".into(), f.clone());
            }
            "stars.add"
        }
        "remove" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("timestamp".into(), sub.get_one::<String>("timestamp").unwrap().clone());
            if let Some(f) = sub.get_one::<String>("file") {
                params.insert("file".into(), f.clone());
            }
            "stars.remove"
        }
        "list" => {
            params.insert("limit".into(), sub.get_one::<String>("limit").cloned().unwrap_or_else(|| "100".into()));
            if let Some(c) = sub.get_one::<String>("cursor") {
                params.insert("cursor".into(), c.clone());
            }
            "stars.list"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
