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

use std::{env, sync::Arc};

use anyhow::Result;
use futures_util::StreamExt;
use tokio::task::JoinSet;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{
    error::ReceiveMessageError,
    stream::{create_recommended, ShardEventStream},
    Config, ConfigBuilder, Event, Intents, Shard,
};
use twilight_http::Client;

const INTENTS: Intents = Intents::empty();

pub struct Api {
    pub client: Arc<Client>,
    pub cache: Arc<InMemoryCache>,
}

impl Api {
    pub fn as_ref(&self) -> ApiRef {
        ApiRef {
            client: &self.client,
            cache: &self.cache,
        }
    }

    pub fn as_mut(&mut self) -> ApiMut {
        ApiMut {
            client: &mut self.client,
            cache: &mut self.cache,
        }
    }
}

pub struct ApiMut<'api> {
    pub client: &'api mut Arc<Client>,
    pub cache: &'api mut Arc<InMemoryCache>,
}

impl ApiMut<'_> {
    pub fn into_owned(self) -> Api {
        Api {
            client: Arc::clone(self.client),
            cache: Arc::clone(self.cache),
        }
    }
}

pub struct ApiRef<'api> {
    pub client: &'api Arc<Client>,
    pub cache: &'api Arc<InMemoryCache>,
}

impl ApiRef<'_> {
    pub fn into_owned(self) -> Api {
        Api {
            client: Arc::clone(self.client),
            cache: Arc::clone(self.cache),
        }
    }
}

pub struct Bot {
    api: Api,
    shards: Box<[Shard]>,
}

impl Bot {
    pub async fn new() -> Result<Self> {
        let token = env::var("TOKEN")?;
        let client = Arc::new(Client::new(token.clone()));
        let cache = Arc::new(InMemoryCache::new());
        let api = Api { client, cache };

        let config = Self::new_config(token)?;
        let shards = create_recommended(&api.client, config, |_, b| b.build())
            .await?
            .collect();

        Ok(Self { api, shards })
    }

    fn new_config(token: String) -> Result<Config> {
        Ok(ConfigBuilder::new(token, INTENTS).build())
    }

    pub async fn run(mut self) -> Result<()> {
        let mut stream = ShardEventStream::new(self.shards.iter_mut());
        let mut tasks = JoinSet::new();

        while let Some((_, event)) = stream.next().await {
            if Self::handle_event(self.api.as_ref(), &mut tasks, event).is_err() {
                break;
            };
        }

        drop(stream);

        while tasks.join_next().await.is_some() {}

        Ok(())
    }

    fn handle_event(
        api: ApiRef<'_>,
        tasks: &mut JoinSet<Result<()>>,
        event: Result<Event, ReceiveMessageError>,
    ) -> Result<()> {
        let event = match event {
            Ok(event) => event,

            Err(error) if error.is_fatal() => {
                eprintln!("{error}");

                return Err(error.into());
            }

            Err(error) => {
                eprintln!("{error}");

                return Ok(());
            }
        };

        api.cache.update(&event);

        tasks.spawn(handle_event_task(api.into_owned(), event));

        Ok(())
    }
}

async fn handle_event_task(api: Api, event: Event) -> Result<()> {
    Ok(())
}
