pub mod entry;

use self::entry::Entry;
use anyhow::Result;

const CSV_URI: &str = "https://raw.githubusercontent.com/COVID19-Malta/COVID19-Data/master/COVID-19%20Malta%20-%20COVID%20Tests.csv";

pub async fn fetch_latest_case() -> Result<Entry> {
    let req = reqwest::get(CSV_URI).await?;
    let content = req.text().await?;
    let content: Vec<&str> = content.trim().split("\r\n").collect();

    Ok(entry::parse_from_csv(content.last().unwrap())?)
}
