mod covid;

use anyhow::Result;
use dotenv::dotenv;
use std::{thread, time::Duration};
use teloxide::{
    prelude::Requester,
    prelude2::{Bot, RequesterExt},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let channel_id = std::env::var("CHANNEL_ID").unwrap();
    let bot = Bot::from_env().auto_send();
    let mut is_sent = false;

    loop {
        let latest_case = covid::fetch_latest_case().await?;

        if !is_sent {
            bot.send_message(channel_id.clone(), latest_case.into_pretty_string())
                .await?;

            is_sent = true;
        } else if is_sent {
            is_sent = false;
        }

        thread::sleep(Duration::from_secs(60));
    }
}
