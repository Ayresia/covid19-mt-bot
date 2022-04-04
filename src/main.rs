mod covid;

use anyhow::Result;
use chrono::{NaiveDate, Local};
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
    let mut last_sent_date: Option<NaiveDate> = None;

    loop {
        let current_date = Local::now().date().naive_local();

        if last_sent_date.is_none() {
            let latest_case = covid::fetch_latest_case().await?;

            if latest_case.date == current_date {
                bot.send_message(channel_id.clone(), latest_case.into_pretty_string())
                    .await?;

                last_sent_date = Some(current_date);
            }
        } else if let Some(date) = last_sent_date {
            if date != current_date {
                last_sent_date = None;
            }
        }

        thread::sleep(Duration::from_secs(60));
    }
}
