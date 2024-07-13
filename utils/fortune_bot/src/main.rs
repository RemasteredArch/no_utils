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
use bot::Bot;
use futures_util::{future::select, pin_mut};

mod bot;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let bot = Bot::new().await?;

    let run = bot.run();
    let interupt = tokio::signal::ctrl_c();

    // Freeze locations in memory and make mutable
    pin_mut!(run);
    pin_mut!(interupt);

    // Wait for whichever one ends first
    select(run, interupt).await;

    Ok(())
}
