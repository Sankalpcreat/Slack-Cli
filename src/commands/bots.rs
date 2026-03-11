use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("bots")
        .about("Bot users")
        .subcommand(Command::new("info").about("Get bot info").arg(clap::arg!(<bot_id>)))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    if name != "info" {
        return Ok(());
    }
    let mut params = HashMap::new();
    params.insert("bot".into(), sub.get_one::<String>("bot_id").unwrap().clone());
    let out = client::call(token, "bots.info", Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
