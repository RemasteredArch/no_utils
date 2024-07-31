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

/// Defines the command schema
pub fn new(guild_id: Option<Id<GuildMarker>>) -> Option<Command> {
    // In debug mode, only register commands in the provided server (or nowhere in none is provided)
    if cfg!(debug_assertions) && guild_id.is_none() {
        return None;
    }

    // Build a new slash command
    let mut builder = CommandBuilder::new("fortune", "Tells a fortune", CommandType::ChatInput);

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
                .content(fortune().await?)
                .build(),
        ),
    };

    client
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    Ok(())
}

/// Returns a random fortune.
///
/// Uses a shell call to `fortune`.
async fn fortune() -> Result<Box<str>> {
    Ok(shell_call("fortune").await?.join("\n").into_boxed_str())
}

/// Make a shell call.
///
/// On Windows:
/// ```cmd
/// cmd /C command
/// ```
///
/// On POSIX:
/// ```sh
/// sh -c command
/// ```
async fn shell_call(command: impl AsRef<str>) -> Result<Vec<Box<str>>> {
    let command = command.as_ref();

    let output = if cfg!(target_os = "windows") {
        tokio::process::Command::new("cmd")
            .args(["/C", command])
            .output()
            .await?
    } else {
        tokio::process::Command::new("sh")
            .args(["-c", command])
            .output()
            .await?
    };

    // Split into lines
    // Is this broken by Windows' CRLF?
    let output = output.stdout.split(|c| *c == b'\n');

    // Parse each line from bytes into strings
    // What can I do about that unwrap?
    let lines = output.map(|s| std::str::from_utf8(s).unwrap().into());

    Ok(lines.collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shell_call() {
        assert_eq!(
            *shell_call("echo test").await.unwrap().first().unwrap(),
            "test".into()
        );
    }

    #[tokio::test]
    async fn test_fortune() {
        assert!(fortune().await.unwrap().len() > 0);
    }
}
