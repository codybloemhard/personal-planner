use chrono::prelude::*;

use super::astr;
use super::save;
use super::astr::AStr;

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

    pub fn make_date(dmy: DMY) -> Result<DT, ()>{
        let date = Local.ymd_opt(dmy.2 as i32, dmy.1, dmy.0).and_hms_opt(0, 0, 0);
        if date == chrono::LocalResult::None { return Err(()); }
        return Ok(DT{ dt: date.unwrap(), });
    }

    pub fn make_datetime(dmy: DMY, hms: HMS) -> Result<DT, ()>{
        let datetime = Local.ymd_opt(dmy.2 as i32, dmy.1, dmy.0).and_hms_opt(hms.0, hms.1, hms.2);
        if datetime == chrono::LocalResult::None { return Err(()); }
        return Ok(DT{ dt: datetime.unwrap(), });
    }

    pub fn str_datetime(&self) -> String{    
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

impl save::Bufferable for DT{
    type Return = DT;
    fn into_buffer(&self, vec: &mut Vec<u8>){
        u32::into_buffer(&self.dt.hour(), vec);
        u32::into_buffer(&self.dt.minute(), vec);
        u32::into_buffer(&self.dt.second(), vec);
        u32::into_buffer(&self.dt.day(), vec);
        u32::into_buffer(&self.dt.month(), vec);
        u32::into_buffer(&(self.dt.year() as u32), vec);
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Result<DT,()>{
        if (vec.len() as i32) - (*iter as i32) < 24 { return Err(()); }
        //we can unwrap without check, buffer_read_u32 only fails if not enough bytes
        //we have checked there are enough bytes
        let ho = u32::from_buffer(vec, iter).unwrap();
        let mi = u32::from_buffer(vec, iter).unwrap();
        let se = u32::from_buffer(vec, iter).unwrap();
        let da = u32::from_buffer(vec, iter).unwrap();
        let mo = u32::from_buffer(vec, iter).unwrap();
        let ye = u32::from_buffer(vec, iter).unwrap();
        return DT::make_datetime((da,mo,ye), (ho,mi,se));
    }
}

pub fn parse_dmy_or_hms(string: &astr::Astr) -> Result<DMY, ()>{
    let splitted = string.split_str(&astr::from_str(":;-_.,/\\"));
    if splitted.len() != 3 { return Err(()); }
    let triplet: Vec<u32> = splitted.iter().map(astr::to_u32_unchecked).collect();
    return Ok((triplet[0],triplet[1],triplet[2]));
}