use chrono::prelude::*;

use super::astr;
use super::save;
use super::astr::AStr;

type DMY = (u32,u32,u32);
type HMS = (u32,u32,u32);

pub struct Span {
    pub total_hours: u64,
    pub total_mins: u64,
    pub total_secs: u64,
    pub days: u64,
    pub hours: u64,
    pub mins: u64,
    pub secs: u64,
    pub neg: bool,
}

impl Span {
    pub const SECS_MIN: u64 = 60;
    pub const SECS_HOUR: u64 = 3600;
    pub const SECS_DAY: u64 = 86400;

    pub fn string_normal(&self) -> String{
        return format!("{}s:{}m:{}h-{}d", self.secs, self.mins, self.hours, self.days);
    }

    pub fn string_significant(&self) -> String{
        let prefix = match self.neg{
            true => "past ",
            false => "in ",
        };
        if self.total_hours > 48 {
            return format!("{}{} days", prefix, self.days);
        }
        if self.total_mins > 60 {
            return format!("{}{} hours", prefix, self.total_hours);
        }
        if self.total_secs > 60 {
            return format!("{}{} mins", prefix, self.total_mins);
        }
        return format!("{}{} secs", prefix, self.total_secs);
    }
}

#[derive(Eq)]
pub struct DT {
    pub dt: chrono::DateTime<Local>,
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

    pub fn str_dayofweek(&self) -> String{
        match self.dt.weekday(){
            chrono::Weekday::Mon => String::from("Monday"),
            chrono::Weekday::Tue => String::from("Tuesday"),
            chrono::Weekday::Wed => String::from("Wednesday"),
            chrono::Weekday::Thu => String::from("Thursday"),
            chrono::Weekday::Fri => String::from("Friday"),
            chrono::Weekday::Sat => String::from("Saturday"),
            chrono::Weekday::Sun => String::from("Sunday"),
        }
    }

    pub fn add(&mut self, days: i64, months: i64, years: i64){
        self.dt = self.dt + chrono::Duration::days(days + (months * 30) + (years * 365));
    }

    pub fn diff(&self, other: &DT) -> Span{
        fn _diff(secs_all: u64, neg: bool) -> Span{
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
                neg: neg,
            };
        }
        fn get_secs(me: &DT, other: &DT) -> u64{
            let d = other.dt - me.dt;
            let stdd = d.to_std();
            match stdd{
                Ok(x) => x.as_secs(),
                Err(_) => 0,
            }
        }
        let mut secs = get_secs(self, other);
        let mut neg = false;
        if secs == 0{
            secs = get_secs(other, self);
            neg = true;
        }
        return _diff(secs, neg);
    }
}

impl save::Bufferable for DT{
    fn into_buffer(&self, vec: &mut Vec<u8>){
        u8::into_buffer(&(self.dt.hour() as u8), vec);
        u8::into_buffer(&(self.dt.minute() as u8), vec);
        u8::into_buffer(&(self.dt.second() as u8), vec);
        u8::into_buffer(&(self.dt.day() as u8), vec);
        u8::into_buffer(&(self.dt.month() as u8), vec);
        u32::into_buffer(&(self.dt.year() as u32), vec);
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Result<Self,()>{
        if (vec.len() as i32) - (*iter as i32) < 9 { return Err(()); }
        //we can unwrap without check, buffer_read_u32 only fails if not enough bytes
        //we have checked there are enough bytes
        let ho = u8::from_buffer(vec, iter).unwrap() as u32;
        let mi = u8::from_buffer(vec, iter).unwrap() as u32;
        let se = u8::from_buffer(vec, iter).unwrap() as u32;
        let da = u8::from_buffer(vec, iter).unwrap() as u32;
        let mo = u8::from_buffer(vec, iter).unwrap() as u32;
        let ye = u32::from_buffer(vec, iter).unwrap();
        return DT::make_datetime((da,mo,ye), (ho,mi,se));
    }
}


impl std::cmp::Ord for DT {
    fn cmp(&self, other: &DT) -> std::cmp::Ordering {
        return self.dt.cmp(&other.dt);
    }
}

impl std::cmp::PartialOrd for DT {
    fn partial_cmp(&self, other: &DT) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl std::cmp::PartialEq for DT {
    fn eq(&self, other: &DT) -> bool {
        return self.dt == other.dt;
    }
}

pub fn parse_dmy_or_hms(string: &astr::Astr) -> Result<DMY, ()>{
    let splitted = string.split_str(&astr::from_str(":;-_.,/\\"));
    if splitted.len() != 3 { return Err(()); }
    let triplet: Vec<u32> = splitted.iter().map(astr::to_u32_unchecked).collect();
    return Ok((triplet[0],triplet[1],triplet[2]));
}

#[derive(Eq)]
pub struct Point{
    pub dt: DT,
    pub title: astr::Astr,
    pub is_deadline: bool,
}

impl Point{
    pub fn new(dt: DT, title: astr::Astr, is_deadline: bool) -> Self{
        Point{
            dt: dt,
            title: title,
            is_deadline: is_deadline,
        }
    }
}

impl save::Bufferable for Point{
    fn into_buffer(&self, vec: &mut Vec<u8>){
        self.title.into_buffer(vec);
        self.dt.into_buffer(vec);
        match self.is_deadline{
            true => 1 as u8,
            false => 0 as u8,
        }.into_buffer(vec);
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Result<Self,()>{
        let res_title = astr::Astr::from_buffer(vec, iter);
        if res_title.is_err() {return Err(());}
        let res_dt = DT::from_buffer(vec, iter);
        if res_dt.is_err() {return Err(());}
        let res_isdead = u8::from_buffer(vec, iter);
        if res_isdead.is_err() {return Err(());}
        return Ok(Point{
            title: res_title.unwrap(),
            dt: res_dt.unwrap(),
            is_deadline: res_isdead.unwrap() != 0,
        }); 
    }
}

impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Point) -> std::cmp::Ordering {
        return self.dt.cmp(&other.dt);
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.dt == other.dt;
    }
}
