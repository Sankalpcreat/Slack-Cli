use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("chat")
        .about("Chat and messaging")
        .subcommand(
            Command::new("post")
                .about("Post a message to a channel")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<text> ...))
                .arg(clap::arg!(--thread <TS> "Thread ts for reply")),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a message")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<ts>)),
        )
        .subcommand(
            Command::new("update")
                .about("Update a message")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<ts>))
                .arg(clap::arg!(<text> ...)),
        )
        .subcommand(
            Command::new("schedule")
                .about("Schedule a message")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<post_at_unix_ts>))
                .arg(clap::arg!(<text> ...)),
        )
        .subcommand(
            Command::new("scheduled")
                .about("List scheduled messages")
                .arg(clap::arg!([channel])),
        )
        .subcommand(
            Command::new("delete-scheduled")
                .about("Delete a scheduled message")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<scheduled_message_id>)),
        )
        .subcommand(
            Command::new("permalink")
                .about("Get permalink for a message")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<ts>)),
        )
        .subcommand(
            Command::new("ephemeral")
                .about("Post ephemeral message")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<user>))
                .arg(clap::arg!(<text> ...)),
        )
        .subcommand(
            Command::new("me")
                .about("Post /me message")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<text> ...)),
        )
        .subcommand(
            Command::new("unfurl")
                .about("Provide custom unfurl for URLs")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<ts>))
                .arg(clap::arg!(--unfurls <JSON> "JSON map of URL to unfurl attachment").required(true)),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    match m.subcommand() {
        Some(("post", sm)) => {
            let mut params = HashMap::new();
            params.insert("channel".into(), sm.get_one::<String>("channel").unwrap().clone());
            params.insert("text".into(), sm.get_many::<String>("text").unwrap().cloned().collect::<Vec<_>>().join(" "));
            if let Some(t) = sm.get_one::<String>("thread") {
                params.insert("thread_ts".into(), t.clone());
            }
            let out = client::call(token, "chat.postMessage", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("delete", sm)) => {
            let mut params = HashMap::new();
            params.insert("channel".into(), sm.get_one::<String>("channel").unwrap().clone());
            params.insert("ts".into(), sm.get_one::<String>("ts").unwrap().clone());
            let out = client::call(token, "chat.delete", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("update", sm)) => {
            let mut params = HashMap::new();
            params.insert("channel".into(), sm.get_one::<String>("channel").unwrap().clone());
            params.insert("ts".into(), sm.get_one::<String>("ts").unwrap().clone());
            params.insert("text".into(), sm.get_many::<String>("text").unwrap().cloned().collect::<Vec<_>>().join(" "));
            let out = client::call(token, "chat.update", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("schedule", sm)) => {
            let mut params = HashMap::new();
            params.insert("channel".into(), sm.get_one::<String>("channel").unwrap().clone());
            params.insert("post_at".into(), sm.get_one::<String>("post_at_unix_ts").unwrap().clone());
            params.insert("text".into(), sm.get_many::<String>("text").unwrap().cloned().collect::<Vec<_>>().join(" "));
            let out = client::call(token, "chat.scheduleMessage", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("scheduled", sm)) => {
            let mut params = HashMap::new();
            if let Some(c) = sm.get_one::<String>("channel") {
                params.insert("channel".into(), c.clone());
            }
            let out = client::call(token, "chat.scheduledMessages.list", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("delete-scheduled", sm)) => {
            let mut params = HashMap::new();
            params.insert("channel".into(), sm.get_one::<String>("channel").unwrap().clone());
            params.insert("scheduled_message_id".into(), sm.get_one::<String>("scheduled_message_id").unwrap().clone());
            let out = client::call(token, "chat.deleteScheduledMessage", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("permalink", sm)) => {
            let mut params = HashMap::new();
            params.insert("channel".into(), sm.get_one::<String>("channel").unwrap().clone());
            params.insert("message_ts".into(), sm.get_one::<String>("ts").unwrap().clone());
            let out = client::call(token, "chat.getPermalink", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("ephemeral", sm)) => {
            let mut params = HashMap::new();
            params.insert("channel".into(), sm.get_one::<String>("channel").unwrap().clone());
            params.insert("user".into(), sm.get_one::<String>("user").unwrap().clone());
            params.insert("text".into(), sm.get_many::<String>("text").unwrap().cloned().collect::<Vec<_>>().join(" "));
            let out = client::call(token, "chat.postEphemeral", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("me", sm)) => {
            let mut params = HashMap::new();
            params.insert("channel".into(), sm.get_one::<String>("channel").unwrap().clone());
            params.insert("text".into(), sm.get_many::<String>("text").unwrap().cloned().collect::<Vec<_>>().join(" "));
            let out = client::call(token, "chat.meMessage", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        Some(("unfurl", sm)) => {
            let mut params = HashMap::new();
            params.insert("channel".into(), sm.get_one::<String>("channel").unwrap().clone());
            params.insert("ts".into(), sm.get_one::<String>("ts").unwrap().clone());
            params.insert("unfurls".into(), sm.get_one::<String>("unfurls").unwrap().clone());
            let out = client::call(token, "chat.unfurl", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        _ => {}
    }
    Ok(())
}
