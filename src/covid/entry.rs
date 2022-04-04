use anyhow::Result;
use chrono::NaiveDate;

pub struct Entry {
    date: NaiveDate,
    new_cases: u32,
    total_cases: u32,
    recovered: u32,
    deaths: u32,
    active_cases: u32,
}

pub fn parse_from_csv(row: &str) -> Result<Entry> {
    let mut iter_row = row.split(',').collect::<Vec<&str>>().into_iter();

    Ok(Entry {
        date: parse_date(iter_row.next().unwrap())?,
        new_cases: iter_row.next().unwrap().parse()?,
        total_cases: iter_row.next().unwrap().parse()?,
        recovered: iter_row.next().unwrap().parse()?,
        deaths: iter_row.next().unwrap().parse()?,
        active_cases: iter_row.next().unwrap().parse()?,
    })
}

fn parse_date(date: &str) -> Result<NaiveDate> {
    Ok(NaiveDate::parse_from_str(date, "%d/%m/%Y")?)
}
