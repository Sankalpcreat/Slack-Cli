use crate::client;
use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("emoji")
        .about("Workspace emoji")
        .subcommand(
            Command::new("list")
                .about("List workspace emoji")
                .arg(Arg::new("include-categories").long("include-categories").action(ArgAction::SetTrue)),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    if name != "list" {
        return Ok(());
    }
    let mut params = HashMap::new();
    if sub.get_flag("include-categories") {
        params.insert("include_categories".into(), "true".into());
    }
    let out = client::call(token, "emoji.list", Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
