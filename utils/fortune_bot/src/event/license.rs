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

use anyhow::Result;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::Interaction,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::GuildMarker, Id},
};
use twilight_util::builder::{command::CommandBuilder, InteractionResponseDataBuilder};

use crate::bot::ApiRef;

/// Self-documenting
const MESSAGE: &str = r#"
fortune_bot is a part of no_utils. no_utils is licensed under the GNU Affero General Public License version 3, or (at your option) any later version. You should have received a copy of the GNU Affero General Public License along with no_utils, found in [`../../LICENSE`](https://github.com/RemasteredArch/no_utils/blob/main/LICENSE). If not, see https://www.gnu.org/licenses/.
"#;

/// Defines the command schema
pub fn new(guild_id: Option<Id<GuildMarker>>) -> Option<Command> {
    // In debug mode, only register commands in the provided server (or nowhere in none is provided)
    if cfg!(debug_assertions) && guild_id.is_none() {
        return None;
    }

    // Build a new slash command
    let mut builder = CommandBuilder::new(
        "license",
        "Display the license of fortune_bot",
        CommandType::ChatInput,
    );

    // Assign it a particular guild ID if it exists (otherwise register globally)
    if let Some(guild_id) = guild_id {
        builder = builder.guild_id(guild_id);
    }

    Some(builder.build())
}

/// Executes the command
pub async fn call(api: ApiRef<'_>, interaction: &Interaction) -> Result<()> {
    let client = api.client.interaction(interaction.application_id);

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .content(MESSAGE.trim())
                .build(),
        ),
    };

    client
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    Ok(())
}
