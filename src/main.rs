// Copyright 2020-2022 The NATS Authors
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bytes::Bytes;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::time::Instant;

extern crate dotenv;

use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct LoginPayload {
    login: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request<T> {
    pattern: String,
    data: T,
}

#[derive(Serialize, Debug)]
struct Token {
    token: String,
}
#[derive(Serialize, Debug)]
struct Response<T> {
    body: T,
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    dotenv().ok();

    let client = async_nats::connect(dotenv::var("NATS").unwrap()).await?;

    let mut subscriber = client.subscribe("foobar".into()).await.unwrap();

    println!("Awaiting messages");

    while let Some(message) = subscriber.next().await {
        let now = Instant::now();
        let str = String::from_utf8(message.payload.to_vec());
        let input: Request<LoginPayload> = serde_json::from_str(&str.unwrap()).unwrap();
        println!("Received login {:?}", message);
        println!("Received password {:?}", input.data.password);
        let response = Response {
            body: Token {
                token: "rusttoken".into(),
            },
        };
        match message.reply {
            Some(reply) => {
                client
                    .publish(
                        reply,
                        Bytes::from(serde_json::to_string(&response).unwrap()),
                    )
                    .await
                    .unwrap();
            }
            None => {
                println!("No reply topic")
            }
        }
        println!("subscriber work in {:?}", now.elapsed());
    }

    Ok(())
}
