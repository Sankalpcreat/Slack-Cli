use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("api")
        .about("Call any Slack Web API method")
        .arg(clap::arg!(<method> "API method (e.g. chat.postMessage)"))
        .arg(clap::arg!(-p --param <PAIR> "Param as key=value").num_args(1..))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let method = m.get_one::<String>("method").unwrap();
    let mut params = HashMap::new();
    for p in m.get_many::<String>("param").unwrap_or_default() {
        if let Some((k, v)) = p.split_once('=') {
            params.insert(k.to_string(), v.trim_matches('"').to_string());
        }
    }
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
