use chrono::prelude::*;

use super::astr;

type DMY = (u32,u32,u32);
type HMS = (u32,u32,u32);

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
    pub const SECS_MIN: u64 = 60;
    pub const SECS_HOUR: u64 = 3600;
    pub const SECS_DAY: u64 = 86400;

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

    pub fn make_date(dmy: DMY) -> Result<DT, u8>{
        let date = Local.ymd_opt(dmy.2 as i32, dmy.1, dmy.0).and_hms_opt(0, 0, 0);
        if date == chrono::LocalResult::None { return Err(0); }
        return Ok(DT{ dt: date.unwrap(), });
    }

    pub fn make_datetime(dmy: DMY, hms: HMS) -> Result<DT, u8>{
        let datetime = Local.ymd_opt(dmy.2 as i32, dmy.1, dmy.0).and_hms_opt(hms.0, hms.1, hms.2);
        if datetime == chrono::LocalResult::None { return Err(0); }
        return Ok(DT{ dt: datetime.unwrap(), });
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

pub fn parse_dmy_or_hms(string: &astr::Astr) -> Result<DMY, u8>{
    let splitted = astr::split(&string, &astr::from_str(":;-_.,/\\"));
    if splitted.len() != 3 { return Err(0); }
    let triplet: Vec<u32> = splitted.iter().map(astr::to_u32_unchecked).collect();
    return Ok((triplet[0],triplet[1],triplet[2]));
}