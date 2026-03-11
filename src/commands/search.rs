use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("search")
        .about("Search")
        .arg(clap::arg!(-q --query <Q> "Search query"))
        .arg(clap::arg!(--count <N>).default_value("20"))
        .arg(clap::arg!(--sort <S>).default_value("score"))
        .subcommand(Command::new("all").about("Search all"))
        .subcommand(Command::new("messages").about("Search messages"))
        .subcommand(Command::new("files").about("Search files"))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let mut params = HashMap::new();
    if let Some(q) = m.get_one::<String>("query") {
        params.insert("query".into(), q.clone());
    }
    params.insert("count".into(), m.get_one::<String>("count").cloned().unwrap_or_else(|| "20".into()));
    params.insert("sort".into(), m.get_one::<String>("sort").cloned().unwrap_or_else(|| "score".into()));

    let method = match m.subcommand() {
        Some(("all", _)) => "search.all",
        Some(("messages", _)) => "search.messages",
        Some(("files", _)) => "search.files",
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
