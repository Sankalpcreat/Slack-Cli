use crate::client;
use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::collections::HashMap;

fn cmd() -> Command {
    Command::new("conversations")
        .about("Channels and conversations")
        .subcommand(
            Command::new("list")
                .about("List channels")
                .arg(Arg::new("exclude-archived").long("exclude-archived").action(ArgAction::SetTrue))
                .arg(clap::arg!(--types <T> "Channel types").default_value("public_channel,private_channel"))
                .arg(clap::arg!(--limit <N>).default_value("100")),
        )
        .subcommand(
            Command::new("history")
                .about("Get channel history")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(--limit <N>).default_value("100"))
                .arg(clap::arg!(--oldest <TS>))
                .arg(clap::arg!(--latest <TS>)),
        )
        .subcommand(
            Command::new("replies")
                .about("Get thread replies")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<ts>))
                .arg(clap::arg!(--limit <N>).default_value("100")),
        )
        .subcommand(Command::new("info").about("Get channel info").arg(clap::arg!(<channel>)))
        .subcommand(Command::new("create").about("Create a channel").arg(clap::arg!(<name>)).arg(Arg::new("private").long("private").action(ArgAction::SetTrue)))
        .subcommand(Command::new("archive").about("Archive a channel").arg(clap::arg!(<channel>)))
        .subcommand(Command::new("unarchive").about("Unarchive a channel").arg(clap::arg!(<channel>)))
        .subcommand(
            Command::new("invite")
                .about("Invite users to a channel")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<user_id> ...)),
        )
        .subcommand(Command::new("join").about("Join a channel").arg(clap::arg!(<channel>)))
        .subcommand(Command::new("leave").about("Leave a channel").arg(clap::arg!(<channel>)))
        .subcommand(
            Command::new("kick")
                .about("Remove user from channel")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<user_id>)),
        )
        .subcommand(
            Command::new("rename")
                .about("Rename a channel")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<name>)),
        )
        .subcommand(Command::new("members").about("List channel members").arg(clap::arg!(<channel>)))
        .subcommand(Command::new("open").about("Open DM or MPIM").arg(clap::arg!([user_id])))
        .subcommand(Command::new("close").about("Close a DM or MPIM").arg(clap::arg!(<channel>)))
        .subcommand(
            Command::new("set-topic")
                .about("Set channel topic")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<topic> ...)),
        )
        .subcommand(
            Command::new("set-purpose")
                .about("Set channel purpose")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<purpose> ...)),
        )
        .subcommand(
            Command::new("mark")
                .about("Mark channel as read")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<ts>)),
        )
        .subcommand(
            Command::new("accept-shared-invite")
                .about("Accept Slack Connect invite")
                .arg(clap::arg!(<invite_id>))
                .arg(clap::arg!(<channel_name>)),
        )
        .subcommand(
            Command::new("approve-shared-invite")
                .about("Approve Slack Connect invite")
                .arg(clap::arg!(<invite_id>)),
        )
        .subcommand(
            Command::new("decline-shared-invite")
                .about("Decline Slack Connect invite")
                .arg(clap::arg!(<invite_id>)),
        )
        .subcommand(
            Command::new("invite-shared")
                .about("Invite to Slack Connect channel")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<emails> ...)),
        )
        .subcommand(
            Command::new("list-connect-invites")
                .about("List Slack Connect invites")
                .arg(clap::arg!([channel_id]))
                .arg(clap::arg!([team_ids])),
        )
}

pub fn command() -> Command {
    cmd()
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let Some((name, sub)) = m.subcommand() else { return Ok(()); };
    let mut params = HashMap::new();
    let method = match name {
        "list" => {
            params.insert("types".into(), sub.get_one::<String>("types").cloned().unwrap_or_else(|| "public_channel,private_channel".into()));
            params.insert("limit".into(), sub.get_one::<String>("limit").cloned().unwrap_or_else(|| "100".into()));
            if sub.get_flag("exclude-archived") {
                params.insert("exclude_archived".into(), "true".into());
            }
            "conversations.list"
        }
        "history" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("limit".into(), sub.get_one::<String>("limit").cloned().unwrap_or_else(|| "100".into()));
            if let Some(o) = sub.get_one::<String>("oldest") {
                params.insert("oldest".into(), o.clone());
            }
            if let Some(l) = sub.get_one::<String>("latest") {
                params.insert("latest".into(), l.clone());
            }
            "conversations.history"
        }
        "replies" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("ts".into(), sub.get_one::<String>("ts").unwrap().clone());
            params.insert("limit".into(), sub.get_one::<String>("limit").cloned().unwrap_or_else(|| "100".into()));
            "conversations.replies"
        }
        "info" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            "conversations.info"
        }
        "create" => {
            params.insert("name".into(), sub.get_one::<String>("name").unwrap().clone());
            if sub.get_flag("private") {
                params.insert("is_private".into(), "true".into());
            }
            "conversations.create"
        }
        "archive" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            "conversations.archive"
        }
        "unarchive" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            "conversations.unarchive"
        }
        "invite" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("users".into(), sub.get_many::<String>("user_id").unwrap().cloned().collect::<Vec<_>>().join(","));
            "conversations.invite"
        }
        "join" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            "conversations.join"
        }
        "leave" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            "conversations.leave"
        }
        "kick" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("user".into(), sub.get_one::<String>("user_id").unwrap().clone());
            "conversations.kick"
        }
        "rename" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("name".into(), sub.get_one::<String>("name").unwrap().clone());
            "conversations.rename"
        }
        "members" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            "conversations.members"
        }
        "open" => {
            if let Some(u) = sub.get_one::<String>("user_id") {
                params.insert("users".into(), u.clone());
            }
            "conversations.open"
        }
        "close" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            "conversations.close"
        }
        "set-topic" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("topic".into(), sub.get_many::<String>("topic").unwrap().cloned().collect::<Vec<_>>().join(" "));
            "conversations.setTopic"
        }
        "set-purpose" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("purpose".into(), sub.get_many::<String>("purpose").unwrap().cloned().collect::<Vec<_>>().join(" "));
            "conversations.setPurpose"
        }
        "mark" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("ts".into(), sub.get_one::<String>("ts").unwrap().clone());
            "conversations.mark"
        }
        "accept-shared-invite" => {
            params.insert("invite_id".into(), sub.get_one::<String>("invite_id").unwrap().clone());
            params.insert("channel_name".into(), sub.get_one::<String>("channel_name").unwrap().clone());
            "conversations.acceptSharedInvite"
        }
        "approve-shared-invite" => {
            params.insert("invite_id".into(), sub.get_one::<String>("invite_id").unwrap().clone());
            "conversations.approveSharedInvite"
        }
        "decline-shared-invite" => {
            params.insert("invite_id".into(), sub.get_one::<String>("invite_id").unwrap().clone());
            "conversations.declineSharedInvite"
        }
        "invite-shared" => {
            params.insert("channel".into(), sub.get_one::<String>("channel").unwrap().clone());
            params.insert("emails".into(), sub.get_many::<String>("emails").unwrap().cloned().collect::<Vec<_>>().join(","));
            "conversations.inviteShared"
        }
        "list-connect-invites" => {
            if let Some(c) = sub.get_one::<String>("channel_id") {
                params.insert("channel_id".into(), c.clone());
            }
            if let Some(t) = sub.get_one::<String>("team_ids") {
                params.insert("team_ids".into(), t.clone());
            }
            "conversations.listConnectInvites"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
