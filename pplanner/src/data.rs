use chrono::prelude::*;
use std::time::*;

pub struct DT {
    dt: chrono::DateTime<Local>,
}

impl DT {
    pub fn new() -> DT {
        DT{
            dt: Local::now(),
        }
    }

    pub fn make_date(day: u32, month: u32, year: i32) -> DT{
        DT{
            dt: Local.ymd(year, month, day).and_hms(0, 0, 0),
        }
    }

    pub fn str_datetime(self) -> String{    
        return format!("{}", self.dt.format("%H:%M:%S %d-%m-%Y"));
    }

    pub fn add(&mut self, days: i64, months: i64, years: i64){
        self.dt = self.dt + chrono::Duration::days(days + (months * 30) + (years * 365));
    }

    pub fn diff(&mut self, other: &DT) -> String{
        let d = other.dt - self.dt;
        let stdd = d.to_std();
        let secs_all = match stdd{
            Ok(x) => x.as_secs(),
            Err(y) => 0,
        };
        let days = secs_all / (60*60*24);
        let mut left = secs_all - (days * (60*60*24));
        let hours = left / (60*60);
        left -= hours * 3600;
        let mins = left / 60;
        left -= mins * 60;
        return format!("{}:{}:{}-{}", left, mins, hours, days);
    }
}
