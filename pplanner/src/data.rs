use chrono::prelude::*;

pub struct Span {
    total_hours: u64,
    total_mins: u64,
    total_secs: u64,
    days: u64,
    hours: u64,
    mins: u64,
    secs: u64,
}

impl Span {
    const SECS_MIN: u64 = 60;
    const SECS_HOUR: u64 = 3600;
    const SECS_DAY: u64 = 86400;

    pub fn string_normal(&mut self) -> String{
        return format!("{}s:{}m:{}h-{}d", self.secs, self.mins, self.hours, self.days);
    }

    pub fn string_significant(&mut self) -> String{
        if self.total_hours > 48 {
            return format!("{} days", self.days);
        }
        if self.total_mins > 60 {
            return format!("{} hours", self.total_hours);
        }
        if self.total_secs > 60 {
            return format!("{} mins", self.total_mins);
        }
        return format!("{} secs", self.total_secs);
    }
}

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

    pub fn diff(&mut self, other: &DT) -> Span{
        let d = other.dt - self.dt;
        let stdd = d.to_std();
        let secs_all = match stdd{
            Ok(x) => x.as_secs(),
            Err(_) => 0,
        };
        let days = secs_all / Span::SECS_DAY;
        let mut left = secs_all - (days * Span::SECS_DAY);
        let hours = left / Span::SECS_HOUR;
        left -= hours * Span::SECS_HOUR;
        let mins = left / Span::SECS_MIN;
        left -= mins * Span::SECS_MIN;

        let total_hours = secs_all / Span::SECS_HOUR;
        let total_mins = secs_all / Span::SECS_MIN;

        return Span {
            total_hours: total_hours,
            total_mins: total_mins,
            total_secs: secs_all,
            days: days,
            hours: hours,
            mins: mins,
            secs: left,
        };
    }
}
