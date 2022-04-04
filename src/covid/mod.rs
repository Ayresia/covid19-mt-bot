pub mod entry;

use self::entry::Entry;
use anyhow::Result;

const CSV_URI: &str = "https://raw.githubusercontent.com/COVID19-Malta/COVID19-Data/master/COVID-19%20Malta%20-%20Aggregate%20Data%20Set.csv";

pub async fn fetch_latest_case() -> Result<Entry> {
    let req = reqwest::get(CSV_URI).await?;
    let content = req.text().await?;
    let content: Vec<&str> = content.trim().split('\n').collect();

    Ok(entry::parse_from_csv(content.last().unwrap())?)
}
