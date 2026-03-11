use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("reminders")
        .about("Reminders")
        .subcommand(Command::new("list").about("List reminders"))
        .subcommand(
            Command::new("add")
                .about("Add reminder")
                .arg(clap::arg!(<text> ...))
                .arg(clap::arg!(--time <T> "Unix ts or 'in 20 minutes'").required(true)),
        )
        .subcommand(Command::new("delete").about("Delete reminder").arg(clap::arg!(<reminder_id>)))
        .subcommand(Command::new("complete").about("Complete reminder").arg(clap::arg!(<reminder_id>)))
        .subcommand(Command::new("info").about("Get reminder info").arg(clap::arg!(<reminder_id>)))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    let mut params = HashMap::new();
    let method = match name {
        "list" => {
            let out = client::call(token, "reminders.list", None)?;
            println!("{}", String::from_utf8_lossy(&out));
            return Ok(());
        }
        "add" => {
            params.insert("text".into(), sub.get_many::<String>("text").unwrap().cloned().collect::<Vec<_>>().join(" "));
            params.insert("time".into(), sub.get_one::<String>("time").unwrap().clone());
            "reminders.add"
        }
        "delete" => {
            params.insert("reminder".into(), sub.get_one::<String>("reminder_id").unwrap().clone());
            "reminders.delete"
        }
        "complete" => {
            params.insert("reminder".into(), sub.get_one::<String>("reminder_id").unwrap().clone());
            "reminders.complete"
        }
        "info" => {
            params.insert("reminder".into(), sub.get_one::<String>("reminder_id").unwrap().clone());
            "reminders.info"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
