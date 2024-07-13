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

use anyhow::Result;
use twilight_gateway::Event;
use twilight_model::gateway::payload::incoming::{InteractionCreate, Ready};

use crate::bot::{Api, ApiRef};

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
    Ok(())
}

// Interactions directly with the bot
// Slash commands, buttons, etc. (not server events like messages)
pub async fn on_interaction(api: ApiRef<'_>, event: InteractionCreate) -> Result<()> {
    Ok(())
}
