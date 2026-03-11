use crate::credentials;

mod api;
mod apps;
mod auth;
mod bookmarks;
mod bots;
mod calls;
mod chat;
mod conversations;
mod dnd;
mod emoji;
mod files;
mod pins;
mod reactions;
mod reminders;
mod search;
mod stars;
mod team;
mod usergroups;
mod users;
mod workflows;

use clap::Command;

pub fn build_cli() -> Command {
    Command::new("slack-api-cli")
        .about("Slack Web API CLI")
        .arg(clap::arg!(-t --team <ID> "Team ID (default: first in credentials)").global(true))
        .subcommand(api::command())
        .subcommand(auth::command())
        .subcommand(chat::command())
        .subcommand(conversations::command())
        .subcommand(conversations::command().name("channels"))
        .subcommand(files::command())
        .subcommand(users::command())
        .subcommand(search::command())
        .subcommand(reactions::command())
        .subcommand(reminders::command())
        .subcommand(pins::command())
        .subcommand(stars::command())
        .subcommand(dnd::command())
        .subcommand(team::command())
        .subcommand(usergroups::command())
        .subcommand(bookmarks::command())
        .subcommand(apps::command())
        .subcommand(emoji::command())
        .subcommand(bots::command())
        .subcommand(workflows::command())
        .subcommand(calls::command())
}

pub fn run(matches: &clap::ArgMatches, team_id: Option<&str>) -> anyhow::Result<()> {
    let token = match matches.subcommand() {
        Some(("auth", m)) if m.subcommand_matches("login").is_some() => {
            m.subcommand_matches("login")
                .and_then(|lm| lm.get_one::<String>("token"))
                .map(|s| s.as_str())
                .ok_or_else(|| anyhow::anyhow!("--token required for auth login"))?
                .to_string()
        }
        _ => credentials::load(team_id)?,
    };
    match matches.subcommand() {
        Some(("api", m)) => api::run(m, &token),
        Some(("auth", m)) => auth::run(m, &token),
        Some(("chat", m)) => chat::run(m, &token),
        Some(("conversations", m)) | Some(("channels", m)) => conversations::run(m, &token),
        Some(("files", m)) => files::run(m, &token),
        Some(("users", m)) => users::run(m, &token),
        Some(("search", m)) => search::run(m, &token),
        Some(("reactions", m)) => reactions::run(m, &token),
        Some(("reminders", m)) => reminders::run(m, &token),
        Some(("pins", m)) => pins::run(m, &token),
        Some(("stars", m)) => stars::run(m, &token),
        Some(("dnd", m)) => dnd::run(m, &token),
        Some(("team", m)) => team::run(m, &token),
        Some(("usergroups", m)) => usergroups::run(m, &token),
        Some(("bookmarks", m)) => bookmarks::run(m, &token),
        Some(("apps", m)) => apps::run(m, team_id),
        Some(("emoji", m)) => emoji::run(m, &token),
        Some(("bots", m)) => bots::run(m, &token),
        Some(("workflows", m)) => workflows::run(m, &token),
        Some(("calls", m)) => calls::run(m, &token),
        _ => Ok(()),
    }
}
