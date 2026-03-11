use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("dnd")
        .about("Do Not Disturb")
        .subcommand(Command::new("info").about("Get DND status").arg(clap::arg!([user])))
        .subcommand(Command::new("snooze").about("Snooze notifications").arg(clap::arg!(--num_minutes <N>)))
        .subcommand(Command::new("end-snooze").about("End snooze"))
        .subcommand(Command::new("end").about("End DND"))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    let mut params = HashMap::new();
    let method = match name {
        "info" => {
            if let Some(u) = sub.get_one::<String>("user") {
                params.insert("user".into(), u.clone());
            }
            "dnd.info"
        }
        "snooze" => {
            if let Some(n) = sub.get_one::<String>("num_minutes") {
                params.insert("num_minutes".into(), n.clone());
            }
            "dnd.setSnooze"
        }
        "end-snooze" => {
            let out = client::call(token, "dnd.endSnooze", None)?;
            println!("{}", String::from_utf8_lossy(&out));
            return Ok(());
        }
        "end" => {
            let out = client::call(token, "dnd.endDnd", None)?;
            println!("{}", String::from_utf8_lossy(&out));
            return Ok(());
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
