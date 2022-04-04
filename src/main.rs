use teloxide::prelude2::*;
use dotenv::dotenv;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let channel_id = std::env::var("CHANNEL_ID")?;
    let bot = Bot::from_env().auto_send();

    Ok(())
}
