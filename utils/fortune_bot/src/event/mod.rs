// SPDX-License-Identifier: AGPL-3.0-or-later
//
// Copyright © 2024 RemasteredArch
//
// This file is part of fortune_bot. fortune_bot is a part of no_utils.
//
// no_utils is free software: you can redistribute it and/or modify it under the terms of the GNU
// Affero General Public License as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// no_utils is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License along with no_utils. If
// not, see <https://www.gnu.org/licenses/>.

#![allow(dead_code)]

use std::str::FromStr;

use anyhow::{bail, Result};
use twilight_gateway::Event;
use twilight_model::{
    application::{
        command::Command,
        interaction::{Interaction, InteractionData, InteractionType},
    },
    gateway::payload::incoming::{InteractionCreate, Ready},
    id::{marker::GuildMarker, Id},
};

use crate::bot::{Api, ApiRef};

pub mod fortune;
pub mod help;

// Handle all events
pub async fn on_event(api: Api, event: Event) -> Result<()> {
    if let Err(error) = match event {
        Event::Ready(event) => on_ready(api.as_ref(), *event).await,
        Event::InteractionCreate(event) => on_interaction(api.as_ref(), *event).await,
        _ => Ok(()), // Ignore unknown event types
    } {
        eprintln!("Failed to handle event: {error}");
    }

    Ok(())
}

// Once bot initializes
pub async fn on_ready(api: ApiRef<'_>, event: Ready) -> Result<()> {
    /// Get a list of commands for the bot
    fn get_commands(guild_id: Option<Id<GuildMarker>>) -> Vec<Command> {
        let mut commands = vec![];

        // Register help
        if let Some(command) = help::new(guild_id) {
            commands.push(command);
        }

        commands
    }

    let client = api.client.interaction(event.application.id);
    let guild_id = Id::<GuildMarker>::from_str(&std::env::var("GUILD_ID")?)?;

    // Set commands for the devlopment server
    client
        .set_guild_commands(guild_id, &get_commands(Some(guild_id)))
        .await?;

    // Set commands for all servers when in release mode
    if cfg!(not(debug_assertions)) {
        client.set_global_commands(&get_commands(None)).await?;
    }

    Ok(())
}

// Interactions directly with the bot
// Slash commands, buttons, etc. (not server events like messages)
pub async fn on_interaction(api: ApiRef<'_>, event: InteractionCreate) -> Result<()> {
    println!("Received interaction: {:?}", event.kind);

    let result = match event.kind {
        InteractionType::ApplicationCommand => on_command(api, &event).await,
        _ => Ok(()), // Ignore other kinds of interactions
    };

    match result.as_ref() {
        Err(error) => eprintln!("Error processing interaction: {error}"),
        Ok(_) => println!("Interaction succeeded!"),
    }

    result
}

pub async fn on_command(api: ApiRef<'_>, event: &Interaction) -> Result<()> {
    dbg!(&event.data);

    // Confirms that this is a ApplicationCommand containing Command Data
    let Some(InteractionData::ApplicationCommand(command)) = event.data.as_ref() else {
        bail!("missing command data");
    };

    match command.name.as_str() {
        "help" => help::call(api, event).await,
        "fortune" => fortune::call(api, event).await,
        unknown => bail!("unknown command '{unknown}'"),
    }
}
