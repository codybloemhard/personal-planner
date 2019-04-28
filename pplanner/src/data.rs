use chrono::prelude::*;
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

use super::conz;
use super::conz::PrinterFunctions;
use super::astr;
use super::save;
use super::astr::AStr;
use super::astr::ToAstr;
use super::misc::{DefaultValue};

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

impl conz::Printable for Span{
    fn print(&self){
        pprint_type!(&"In the past: ", conz::MsgType::Normal);
        if self.neg{
            pprintln_type!(&"Yes", conz::MsgType::Highlight);
        }else{
            pprintln_type!(&"No", conz::MsgType::Highlight);
        }
        pprint_type!(&"Significant: ", conz::MsgType::Normal);
        pprintln_type!(&self.string_significant(), conz::MsgType::Highlight);
        pprint_type!(&"In Seconds: ", conz::MsgType::Normal);
        pprintln_type!(&format!("{}", self.total_secs), conz::MsgType::Highlight);
        pprint_type!(&"In Minutes: ", conz::MsgType::Normal);
        pprintln_type!(&format!("{}", self.total_mins), conz::MsgType::Highlight);
        pprint_type!(&"In Hours: ", conz::MsgType::Normal);
        pprintln_type!(&format!("{}", self.total_hours), conz::MsgType::Highlight);
        pprint_type!(&"In Days: ", conz::MsgType::Normal);
        pprintln_type!(&format!("{}", self.total_hours / 24), conz::MsgType::Highlight);
        pprint_type!(&"In Weeks: ", conz::MsgType::Normal);
        pprintln_type!(&format!("{}", self.total_hours / 168), conz::MsgType::Highlight);
        pprint_type!(&"In Months: ", conz::MsgType::Normal);
        pprintln_type!(&format!("{}", self.total_hours / 720), conz::MsgType::Highlight);
        pprint_type!(&"In Years: ", conz::MsgType::Normal);
        pprintln_type!(&format!("{}", self.total_hours / 8760), conz::MsgType::Highlight);
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

    pub fn make_date(dmy: DMY) -> Option<Self>{
        let date = Local.ymd_opt(dmy.2 as i32, dmy.1, dmy.0).and_hms_opt(0, 0, 0);
        if date == chrono::LocalResult::None {return Option::None;}
        return Option::Some(DT{ dt: date.unwrap(), });
    }

    pub fn make_datetime(dmy: DMY, hms: HMS) -> Option<Self>{
        let datetime = Local.ymd_opt(dmy.2 as i32, dmy.1, dmy.0).and_hms_opt(hms.0, hms.1, hms.2);
        if datetime == chrono::LocalResult::None {return Option::None;}
        return Option::Some(DT{ dt: datetime.unwrap(), });
    }

    pub fn str_datetime(&self) -> astr::Astr{    
        return format!("{}", self.dt.format("%H:%M:%S %d-%m-%Y")).to_astr();
    }

    pub fn str_dayname(&self) -> astr::Astr{
        astr::from_str(match self.dt.weekday(){
            chrono::Weekday::Mon => "Monday",
            chrono::Weekday::Tue => "Tuesday",
            chrono::Weekday::Wed => "Wednesday",
            chrono::Weekday::Thu => "Thursday",
            chrono::Weekday::Fri => "Friday",
            chrono::Weekday::Sat => "Saturday",
            chrono::Weekday::Sun => "Sunday",
        })
    }

    pub fn str_dayname_short(&self) -> astr::Astr{
        astr::from_str(match self.dt.weekday(){
            chrono::Weekday::Mon => "Mon",
            chrono::Weekday::Tue => "Tue",
            chrono::Weekday::Wed => "Wed",
            chrono::Weekday::Thu => "Thu",
            chrono::Weekday::Fri => "Fri",
            chrono::Weekday::Sat => "Sat",
            chrono::Weekday::Sun => "Sun",
        })
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

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Option<Self>{
        if (vec.len() as i32) - (*iter as i32) < 9 { return Option::None; }
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

impl std::clone::Clone for DT{
    fn clone(&self) -> Self{
        DT{
            dt: self.dt.clone(),
        }
    }
}

impl DefaultValue for DT{
    fn default_val() -> Self{
        DT::make_datetime((1,1,1900), (0,0,0)).expect("Expect: DefaultValue for DT")
    }
}

pub fn parse_dmy_or_hms(string: &astr::Astr) -> Option<DMY>{
    let splitted = string.split_str(&astr::from_str(":;-_.,/\\"));
    if splitted.len() != 3 {return Option::None;}
    let triplet: Vec<u32> = splitted.iter().map(astr::to_u32_unchecked).collect();
    return Option::Some((triplet[0],triplet[1],triplet[2]));
}

#[derive(FromPrimitive,ToPrimitive,Eq,Clone)]
pub enum PointType{
    None = 0,
    Deadline = 1,
    Event = 2,
    DefaultValue = 255,
}

impl PartialEq for PointType {
    fn eq(&self, other: &PointType) -> bool {
        ToPrimitive::to_u8(self) == ToPrimitive::to_u8(other)
    }
}

impl PointType{
    pub fn from_astr(string: &astr::Astr, partial: bool) -> PointType{
        let string = string.to_lower();
        if string.len() == 0 && partial{
            return PointType::DefaultValue;
        }
        if string.len() < 3{
            return PointType::None;
        }
        if string.cut(3) == astr::from_str("dea"){
            return PointType::Deadline;
        }
        if string.cut(3) == astr::from_str("eve"){
            return PointType::Event;
        }
        return PointType::None;
    }

    pub fn to_astr(&self) -> astr::Astr{
        astr::from_str(match self{
            PointType::None => "None",
            PointType::Deadline => "Deadline",
            PointType::Event => "Event",
            PointType::DefaultValue => "Error",
        })
    }
}

impl DefaultValue for PointType{
    fn default_val() -> Self{
        return PointType::DefaultValue;
    }
}

#[derive(Eq)]
pub struct Point{
    pub dt: DT,
    pub title: astr::Astr,
    pub ptype: PointType,
}

impl Point{
    pub fn new(dt: DT, title: astr::Astr, ptype: astr::Astr) -> Self{
        Point{
            dt: dt,
            title: title,
            ptype: PointType::from_astr(&ptype, false),
        }
    }
}

impl save::Bufferable for Point{
    fn into_buffer(&self, vec: &mut Vec<u8>){
        self.title.into_buffer(vec);
        self.dt.into_buffer(vec);
        let primtype = ToPrimitive::to_u8(&self.ptype);
        if primtype.is_none() {
            pprintln_type!(&"Error: Could not convert PointType to u8.", conz::MsgType::Error);
            (0 as u8).into_buffer(vec);
        }else{
            primtype.unwrap().into_buffer(vec);
        }
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Option<Self>{
        let res_title = astr::Astr::from_buffer(vec, iter);
        if res_title.is_none() {return Option::None;}
        let res_dt = DT::from_buffer(vec, iter);
        if res_dt.is_none() {return Option::None;}
        let res_ptype = u8::from_buffer(vec, iter);
        if res_ptype.is_none() {return Option::None;}
        let res_ptype = FromPrimitive::from_u8(res_ptype.unwrap());
        if res_ptype.is_none() {return Option::None;}
        return Option::Some(Point{
            title: res_title.unwrap(),
            dt: res_dt.unwrap(),
            ptype: res_ptype.unwrap(),
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

impl std::clone::Clone for Point{
    fn clone(&self) -> Self{
        Point{
            dt: self.dt.clone(),
            title: self.title.clone(),
            ptype: self.ptype.clone(),
        }
    }
}

impl conz::Printable for Point{
    fn print(&self){
        pprint_type!(&"Title: ", conz::MsgType::Normal);
        pprintln_type!(&self.title, conz::MsgType::Highlight);
        pprint_type!(&"Type: ", conz::MsgType::Normal);
        pprintln_type!(&self.ptype.to_astr(), conz::MsgType::Highlight);
        pprint_type!(&"time date: ", conz::MsgType::Normal);
        pprint_type!(&self.dt.str_datetime(), conz::MsgType::Value);
        pprint!(&" ");
        pprintln_type!(&self.dt.str_dayname(), conz::MsgType::Value);
    }
}

#[derive(FromPrimitive,ToPrimitive,Eq,Clone)]
pub enum TodoType{
    Todo = 0,
    Longterm = 1,
    Idea = 2,
    DefaultValue = 255,
}

impl TodoType{
    pub fn from_astr(string: &astr::Astr, partial: bool) -> TodoType{
        let string = string.to_lower();
        if string.len() == 0 && partial{
            return TodoType::DefaultValue;
        }
        if string.len() < 1{
            return TodoType::Todo;
        }
        if string[0] == 't' as u8{
            return TodoType::Todo;
        }
        if string[0] == 'l' as u8{
            return TodoType::Longterm;
        }
        if string[0] == 'i' as u8{
            return TodoType::Idea;
        }
        return TodoType::Todo;
    }

    pub fn to_astr(&self) -> astr::Astr{
        astr::from_str(match self{
            TodoType::Todo => "Todo",
            TodoType::Longterm => "Longterm",
            TodoType::Idea => "Idea",
            TodoType::DefaultValue => "Error",
        })
    }
}

impl DefaultValue for TodoType{
    fn default_val() -> Self{
        return TodoType::DefaultValue;
    }
}

impl PartialEq for TodoType {
    fn eq(&self, other: &TodoType) -> bool {
        ToPrimitive::to_u8(self) == ToPrimitive::to_u8(other)
    }
}

#[derive(Eq)]
pub struct Todo{
    title: astr::Astr,
    ttype: TodoType,
    urgency: u16,
}

impl Todo{
    pub fn new(title: astr::Astr, ttype: astr::Astr, urgency: u16) -> Todo{
        Todo{
            title: title,
            ttype: TodoType::from_astr(&ttype, false),
            urgency: urgency,
        }
    }
}

impl save::Bufferable for Todo{
    fn into_buffer(&self, vec: &mut Vec<u8>){
        self.title.into_buffer(vec);
        self.urgency.into_buffer(vec);
        let primtype = ToPrimitive::to_u8(&self.ttype);
        if primtype.is_none() {
            pprintln_type!(&"Error: Could not convert TodoType to u8.", conz::MsgType::Error);
            (0 as u8).into_buffer(vec);
        }else{
            primtype.unwrap().into_buffer(vec);
        }
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Option<Self>{
        let res_title = astr::Astr::from_buffer(vec, iter);
        if res_title.is_none() {return Option::None;}
        let res_urg = u16::from_buffer(vec, iter);
        if res_urg.is_none() {return Option::None;}
        let res_ttype = u8::from_buffer(vec, iter);
        if res_ttype.is_none() {return Option::None;}
        let res_ttype = FromPrimitive::from_u8(res_ttype.unwrap());
        if res_ttype.is_none() {return Option::None;}
        return Option::Some(Todo{
            title: res_title.unwrap(),
            urgency: res_urg.unwrap(),
            ttype: res_ttype.unwrap(),
        }); 
    }
}

impl std::cmp::Ord for Todo {
    fn cmp(&self, other: &Todo) -> std::cmp::Ordering {
        self.urgency.cmp(&other.urgency)
    }
}

impl std::cmp::PartialOrd for Todo {
    fn partial_cmp(&self, other: &Todo) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Todo {
    fn eq(&self, other: &Todo) -> bool {
        self.title == other.title &&
        self.ttype == other.ttype &&
        self.urgency == other.urgency
    }
}

impl std::clone::Clone for Todo{
    fn clone(&self) -> Self{
        Todo{
            title: self.title.clone(),
            ttype: self.ttype.clone(),
            urgency: self.urgency.clone(),
        }
    }
}
