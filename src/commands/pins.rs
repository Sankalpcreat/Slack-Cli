use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("pins")
        .about("Pins")
        .subcommand(Command::new("list").about("List pins").arg(clap::arg!(<channel>)))
        .subcommand(
            Command::new("add")
                .about("Pin message")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<timestamp>)),
        )
        .subcommand(
            Command::new("remove")
                .about("Unpin message")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<timestamp>)),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    let mut params = HashMap::new();
    params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
    let method = match name {
        "list" => "pins.list",
        "add" => {
            params.insert("timestamp".into(), sub.get_one::<String>("timestamp").unwrap().clone());
            "pins.add"
        }
        "remove" => {
            params.insert("timestamp".into(), sub.get_one::<String>("timestamp").unwrap().clone());
            "pins.remove"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
