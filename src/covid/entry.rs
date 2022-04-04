use anyhow::Result;
use chrono::NaiveDate;

pub struct Entry {
    pub date: NaiveDate,
    pub new_cases: u32,
    pub total_cases: u32,
    pub recovered: u32,
    pub deaths: u32,
    pub active_cases: u32,
}

impl Entry {
    pub fn into_pretty_string(&self) -> String {
        format!("COVID19 Stats - {}\n\nNew Cases: {}\nTotal Cases: {}\nRecovered Cases: {}\nDeaths: {}\nActive Cases: {}", 
            self.date.format("%A %-d %B, %C%y"),
            self.new_cases, 
            self.total_cases, 
            self.recovered, 
            self.deaths, 
            self.active_cases
        )
    }
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
