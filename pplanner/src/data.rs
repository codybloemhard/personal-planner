use chrono::prelude::*;
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::TryInto;
use term_basics_linux as tbl;

use super::conz;
use super::astr;
use super::save;
use super::astr::AStr;
use super::astr::ToAstr;
use super::misc::{DefaultValue,UnwrapDefault};
use super::support;
use super::wizard;

type DMY = (u32,u32,u32);
type HMS = (u32,u32,u32);

pub fn day_name(i: u8) -> astr::Astr{
    astr::from_str(match i{
        0 => "Monday",
        1 => "Tuesday",
        2 => "Wednesday",
        3 => "Thursday",
        4 => "Friday",
        5 => "Saturday",
        6 => "Sunday",
        _ => "Error",
    })
}

pub fn day_name_short(i: u8) -> astr::Astr{
    astr::from_str(match i{
        0 => "Mon",
        1 => "Tue",
        2 => "Wed",
        3 => "Thu",
        4 => "Fri",
        5 => "Sat",
        6 => "Sun",
        _ => "Err",
    })
}

pub fn month_name(i: u8) -> astr::Astr{
    astr::from_str(match i{
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Error",
    })
}

pub fn month_name_short(i: u8) -> astr::Astr{
    astr::from_str(match i{
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Err",
    })
}

pub fn month_short_to_uint(mon: &astr::Astr) -> Option<u32>{
    let lower = mon.to_lower();
    match lower.to_string().as_ref() {
        "jan" => Option::Some(1),
        "feb" => Option::Some(2),
        "mar" => Option::Some(3),
        "apr" => Option::Some(4),
        "may" => Option::Some(5),
        "jun" => Option::Some(6),
        "jul" => Option::Some(7),
        "aug" => Option::Some(8),
        "sep" => Option::Some(9),
        "oct" => Option::Some(10),
        "nov" => Option::Some(11),
        "dec" => Option::Some(12),
        _ => Option::None,
    }
}

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

    pub fn string_significant(&self, as_dur: bool) -> String{
        let prefix = if as_dur { "" }
        else if self.neg {"past "}
        else {"in "};
        if self.total_hours > 24 * 9999 {
            return format!("{}{}^2 years", prefix, ((self.days / 365) as f64).sqrt() as u64 + 1);
        }
        if self.total_hours > 24 * 365 {
            return format!("{}{} years", prefix, self.days / 365);
        }
        if self.total_hours > 24 * 99 {
            return format!("{}{} months", prefix, self.days / 30);
        }
        if self.total_hours > 48 {
            return format!("{}{} days", prefix, self.days);
        }
        if self.total_mins > 60 {
            return format!("{}{} hours", prefix, self.total_hours);
        }
        if self.total_secs > 60 {
            return format!("{}{} mins", prefix, self.total_mins);
        }
        format!("{}{} secs", prefix, self.total_secs)
    }

    pub fn print_as_duration(&self){
        conz::print_type("Significant: ", conz::MsgType::Normal);
        conz::println_type(self.string_significant(true), conz::MsgType::Value);
        self.print_all_lengths();
    }

    fn print_all_lengths(&self){
        conz::print_type("In Seconds: ", conz::MsgType::Normal);
        conz::println_type(format!("{}", self.total_secs), conz::MsgType::Value);
        conz::print_type("In Minutes: ", conz::MsgType::Normal);
        conz::println_type(format!("{}", self.total_mins), conz::MsgType::Value);
        conz::print_type("In Hours: ", conz::MsgType::Normal);
        conz::println_type(format!("{}", self.total_hours), conz::MsgType::Value);
        conz::print_type("In Days: ", conz::MsgType::Normal);
        conz::println_type(format!("{}", self.total_hours / 24), conz::MsgType::Value);
        conz::print_type("In Weeks: ", conz::MsgType::Normal);
        conz::println_type(format!("{}", self.total_hours / 168), conz::MsgType::Value);
        conz::print_type("In Months: ", conz::MsgType::Normal);
        conz::println_type(format!("{}", self.total_hours / 720), conz::MsgType::Value);
        conz::print_type("In Years: ", conz::MsgType::Normal);
        conz::println_type(format!("{}", self.total_hours / 8760), conz::MsgType::Value);
    }
}

impl conz::Printable for Span{
    fn print(&self){
        conz::print_type("In the past: ", conz::MsgType::Normal);
        if self.neg{
            conz::println_type("Yes", conz::MsgType::Value);
        }else{
            conz::println_type("No", conz::MsgType::Value);
        }
        conz::print_type("Significant: ", conz::MsgType::Normal);
        conz::println_type(self.string_significant(false), conz::MsgType::Value);
        self.print_all_lengths();
    }
}

#[derive(Eq, Clone)]
pub struct DT {
    pub dt: chrono::DateTime<Local>,
}

impl DT {
    pub fn new() -> DT {
        DT{
            dt: Local::now(),
        }
    }

    pub fn make_datetime(dmy: DMY, hms: HMS) -> Option<Self>{
        let datetime = Local.ymd_opt(dmy.2 as i32, dmy.1, dmy.0).and_hms_opt(hms.0, hms.1, hms.2);
        if datetime == chrono::LocalResult::None {return Option::None;}
        Option::Some(DT{ dt: datetime.unwrap(), })
    }

    pub fn str_datetime(&self) -> astr::Astr{
        format!("{}", self.dt.format("%H:%M:%S %d-%m-%Y")).to_astr()
    }

    pub fn str_date(&self) -> astr::Astr{
        format!("{}", self.dt.format("%d-%m-%Y")).to_astr()
    }

    pub fn str_time(&self) -> astr::Astr{
        format!("{}", self.dt.format("%H:%M:%S")).to_astr()
    }

    pub fn weeknr(&self) -> u8{
        let datetime = Self::make_datetime((1, 1, self.dt.year() as u32), (0, 0, 0));//first day this year
        if datetime.is_none() { return 0; }
        let datetime = datetime.unwrap();
        let since = self.diff(&datetime);
        let weeks = since.days / 7;
        (weeks + 1) as u8 //year starts with week 1 not week 0
    }

    pub fn str_weeknr(&self) -> astr::Astr{
        format!("{}", self.weeknr()).to_astr()
    }

    pub fn str_dayname(&self) -> astr::Astr{
        day_name(match self.dt.weekday(){
            chrono::Weekday::Mon => 0,
            chrono::Weekday::Tue => 1,
            chrono::Weekday::Wed => 2,
            chrono::Weekday::Thu => 3,
            chrono::Weekday::Fri => 4,
            chrono::Weekday::Sat => 5,
            chrono::Weekday::Sun => 6,
        })
    }

    pub fn str_dayname_short(&self) -> astr::Astr{
        day_name_short(match self.dt.weekday(){
            chrono::Weekday::Mon => 0,
            chrono::Weekday::Tue => 1,
            chrono::Weekday::Wed => 2,
            chrono::Weekday::Thu => 3,
            chrono::Weekday::Fri => 4,
            chrono::Weekday::Sat => 5,
            chrono::Weekday::Sun => 6,
        })
    }

    pub fn str_monthname(&self) -> astr::Astr{
        month_name(self.dt.month() as u8)
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

            Span {
                total_hours,
                total_mins,
                total_secs: secs_all,
                days,
                hours,
                mins,
                secs: left,
                neg,
            }
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
        let neg = if secs == 0 { secs = get_secs(other, self); true }
        else { false };
        _diff(secs, neg)
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
        let ho = u32::from(u8::from_buffer(vec, iter).unwrap());
        let mi = u32::from(u8::from_buffer(vec, iter).unwrap());
        let se = u32::from(u8::from_buffer(vec, iter).unwrap());
        let da = u32::from(u8::from_buffer(vec, iter).unwrap());
        let mo = u32::from(u8::from_buffer(vec, iter).unwrap());
        let ye = u32::from_buffer(vec, iter).unwrap();
        DT::make_datetime((da,mo,ye), (ho,mi,se))
    }
}

impl std::cmp::Ord for DT {
    fn cmp(&self, other: &DT) -> std::cmp::Ordering {
        self.dt.cmp(&other.dt)
    }
}

impl std::cmp::PartialOrd for DT {
    fn partial_cmp(&self, other: &DT) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for DT {
    fn eq(&self, other: &DT) -> bool {
        self.dt == other.dt
    }
}

impl DefaultValue for DT{
    fn default_val() -> Self{
        DT::make_datetime((1,1,1900), (0,0,0)).expect("Expect: DefaultValue for DT")
    }
}

pub fn parse_dmy(string: &astr::Astr) -> Option<DMY>{
    if &string.to_string() == "today"{
        let now = DT::new().dt;
        return Option::Some((now.day(), now.month(), now.year().try_into().unwrap()));
    }
    // if &string.to_string() == "tomorrow"{
    //     let now = DT::new().dt;
    //
    //     return Option::Some((tom.day(), tom.month(), tom.year().try_into().unwrap()));
    // }
    let splitted = string.split_str(&astr::from_str(":;-_.,/\\"));
    if splitted.len() != 3 {return Option::None;}
    let mut triplet: Vec<Option<u32>> = splitted.iter().map(astr::to_u32_checked).collect();
    //if its now than put in piece of the current date
    if triplet[0].is_none() && splitted[0].to_string() == "now"{
        let now = DT::new();
        triplet[0] = Option::Some(now.dt.day());
    }
    if triplet[1].is_none() && splitted[1].to_string() == "now"{
        let now = DT::new();
        triplet[1] = Option::Some(now.dt.month());
    }
    if triplet[2].is_none() && splitted[2].to_string() == "now"{
        let now = DT::new();
        triplet[2] = Option::Some(now.dt.year() as u32);
    }
    //months can be inputted with 3 letter month names. still none, fail.
    triplet[0]?;
    if triplet[1].is_none() {
        triplet[1] = month_short_to_uint(&splitted[1]);
        triplet[1]?;
    }
    triplet[2]?;
    Option::Some((triplet[0].unwrap(),triplet[1].unwrap(),triplet[2].unwrap()))
}

pub fn parse_hms(string: &astr::Astr) -> Option<DMY>{
    if &string.to_string() == "dead"{
        return Option::Some((23,59,59));
    }
    if &string.to_string() == "idk"{
        return Option::Some((0,0,1));
    }
    let splitted = string.split_str(&astr::from_str(":;-_.,/\\"));
    if splitted.len() != 3 {return Option::None;}
    let triplet: Vec<Option<u32>> = splitted.iter().map(astr::to_u32_checked).collect();
    triplet[0]?;
    triplet[1]?;
    triplet[2]?;
    Option::Some((triplet[0].unwrap(),triplet[1].unwrap(),triplet[2].unwrap()))
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
        if string.is_empty() && partial{
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
        PointType::None
    }
}

impl astr::ToAstr for PointType{
    fn to_astr(&self) -> astr::Astr{
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
        PointType::DefaultValue
    }
}

#[derive(Eq,Clone)]
pub struct Point{
    pub dt: DT,
    pub title: astr::Astr,
    pub ptype: PointType,
}

impl Point{
    pub fn new(dt: DT, title: astr::Astr, ptype: astr::Astr) -> Self{
        Point{
            dt,
            title,
            ptype: PointType::from_astr(&ptype, false),
        }
    }
}

impl save::Bufferable for Point{
    fn into_buffer(&self, vec: &mut Vec<u8>){
        self.title.into_buffer(vec);
        self.dt.into_buffer(vec);
        let primtype = ToPrimitive::to_u8(&self.ptype);
        if let Some(primtypev) = primtype{
            primtypev.into_buffer(vec);
        }else{
            conz::println_type("Error: Could not convert PointType to u8.", conz::MsgType::Error);
            (0 as u8).into_buffer(vec);
        }
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Option<Self>{
        let res_title = astr::Astr::from_buffer(vec, iter);
        res_title.as_ref()?;
        let res_dt = DT::from_buffer(vec, iter);
        res_dt.as_ref()?;
        let res_ptype = u8::from_buffer(vec, iter);
        res_ptype.as_ref()?;
        let res_ptype = FromPrimitive::from_u8(res_ptype.unwrap());
        res_ptype.as_ref()?;
        Option::Some(Point{
            title: res_title.unwrap(),
            dt: res_dt.unwrap(),
            ptype: res_ptype.unwrap(),
        })
    }
}

impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Point) -> std::cmp::Ordering {
        self.dt.cmp(&other.dt)
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.dt == other.dt
    }
}

impl conz::Printable for Point{
    fn print(&self){
        conz::print_type("Title: ", conz::MsgType::Normal);
        conz::println_type(self.title.disp(), conz::MsgType::Highlight);
        conz::print_type("Type: ", conz::MsgType::Normal);
        conz::println_type(self.ptype.to_astr().disp(), conz::MsgType::Highlight);
        conz::print_type("time date: ", conz::MsgType::Normal);
        conz::print_type(self.dt.str_datetime().disp(), conz::MsgType::Value);
        tbl::print(" ");
        conz::println_type(self.dt.str_dayname().disp(), conz::MsgType::Value);
    }
}

impl conz::PrettyPrintable for Point{
    type ArgType = DT;
    fn pretty_print(&self, arg: &Self::ArgType) -> (astr::AstrVec,Vec<conz::MsgType>){
        let mut text = Vec::new();
        let mut types = Vec::new();
        let diff = arg.diff(&self.dt);
        text.push(self.title.clone());
        text.push(diff.string_significant(false).to_astr());
        text.push(self.dt.str_datetime()
            .concat(astr::from_str(" "))
            .concat(self.dt.str_dayname_short()));
        text.push(self.ptype.to_astr());
        types.push(conz::MsgType::Normal);
        types.push(support::diff_color(&diff));
        types.push(conz::MsgType::Value);
        types.push(conz::MsgType::Normal);
        (text,types)
    }

    fn lengths(_: &Self::ArgType) -> Vec<u16>{
        vec![32,14,23,11]
    }

    fn titles(_: &Self::ArgType) -> Vec<astr::Astr>{
        vec![astr::from_str("Title:"),
            astr::from_str("Relative:"),
            astr::from_str("Time Date:"),
            astr::from_str("Type:"),]
    }
}

impl wizard::Wizardable for Point{
    fn extract(wres: &mut wizard::WizardRes) -> Option<Self>{
        loop{
            let dt_res = wres.get_dt();
            if dt_res.is_none() {break;}
            let title_res = wres.get_text();
            if title_res.is_none() {break;}
            let isdead_res = wres.get_text();
            if isdead_res.is_none() {break;}
            let ret = Point::new(dt_res.unwrap(), title_res.unwrap(), isdead_res.unwrap());
            return Option::Some(ret);
        }
        conz::println_type("Error: could not build point.", conz::MsgType::Error);
        Option::None
    }

    fn get_fields(partial: bool) -> wizard::FieldVec{
        let mut fields = wizard::FieldVec::new();
        if partial{
            fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Partial);
            fields.add(wizard::InputType::Text, astr::from_str("Type: "), wizard::PromptType::Partial);
            fields.add(wizard::InputType::DateTime, astr::from_str("Time date: "), wizard::PromptType::Partial);
        }else{
            fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Once);
            fields.add(wizard::InputType::Text, astr::from_str("Type: "), wizard::PromptType::Once);
            fields.add(wizard::InputType::DateTime, astr::from_str("Time date: "), wizard::PromptType::Reprompt);
        }
        fields
    }

    fn get_partial(wres: &mut wizard::WizardRes) -> Self{
        let ptitle = astr::Astr::unwrap_default(wres.get_text());
        let ptype = PointType::from_astr(&astr::Astr::unwrap_default(wres.get_text()), true);
        let pdt = DT::unwrap_default(wres.get_dt());
        Point{
            dt: pdt,
            title: ptitle,
            ptype,
        }
    }

    fn replace_parts(&mut self, replacements: &Self){
        self.title.replace_if_not_default(replacements.title.clone());
        self.dt.replace_if_not_default(replacements.dt.clone());
        self.ptype.replace_if_not_default(replacements.ptype.clone());
    }

    fn score_againts(&self, other: &Self) -> i32{
        let mut curr_score = 0;
        if self.title == other.title{
            curr_score += 1;
        }
        if self.ptype == other.ptype{
            curr_score += 1;
        }
        if self.dt == other.dt{
            curr_score += 1;
        }
        curr_score
    }

    fn get_name() -> astr::Astr{
        astr::from_str("point")
    }
}

#[derive(FromPrimitive,ToPrimitive,Eq,Clone)]
pub enum PlanType{
    Short,
    Long,
    Idea,
    Current,
    DefaultValue,
}

impl PartialEq for PlanType {
    fn eq(&self, other: &PlanType) -> bool {
        ToPrimitive::to_u8(self) == ToPrimitive::to_u8(other)
    }
}

impl PlanType{
    pub fn from_astr(string: &astr::Astr, partial: bool) -> PlanType{
        let string = string.to_lower();
        if string.is_empty() && partial{
            return PlanType::DefaultValue;
        }
        if string[0] == b'l'{
            return PlanType::Long;
        }
        if string[0] == b'i'{
            return PlanType::Idea;
        }
        if string[0] == b'c'{
            return PlanType::Current;
        }
        PlanType::Short
    }
}

impl astr::ToAstr for PlanType{
    fn to_astr(&self) -> astr::Astr{
        astr::from_str(match self{
            PlanType::Short => "Short",
            PlanType::Long => "Longterm",
            PlanType::Idea => "Idea",
            PlanType::Current => "Current",
            PlanType::DefaultValue => "Error",
        })
    }
}

impl DefaultValue for PlanType{
    fn default_val() -> Self{
        PlanType::DefaultValue
    }
}

impl std::cmp::PartialOrd for PlanType{
    fn partial_cmp(&self, other: &PlanType) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for PlanType{
    fn cmp(&self, other: &PlanType) -> std::cmp::Ordering {
        ToPrimitive::to_u8(self).cmp(&ToPrimitive::to_u8(other))
    }
}

#[derive(Eq,Clone)]
pub struct Plan{
    title: astr::Astr,
    urgency: u16,
    pub ttype: PlanType,
}

impl Plan{
    pub fn new(title: astr::Astr, urgency: u16, strtype: astr::Astr) -> Plan{
        Plan{
            title,
            urgency,
            ttype: PlanType::from_astr(&strtype, false),
        }
    }
}

impl save::Bufferable for Plan{
    fn into_buffer(&self, vec: &mut Vec<u8>){
        self.title.into_buffer(vec);
        self.urgency.into_buffer(vec);
        let primtype = ToPrimitive::to_u8(&self.ttype);
        if let Some(primtypev) = primtype {
            primtypev.into_buffer(vec);
        }else{
            conz::println_type("Error: Could not convert PlanType to u8.", conz::MsgType::Error);
            (0 as u8).into_buffer(vec);
        }
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Option<Self>{
        let res_title = astr::Astr::from_buffer(vec, iter);
        res_title.as_ref()?;
        let res_urg = u16::from_buffer(vec, iter);
        res_urg.as_ref()?;
        let res_ttype = u8::from_buffer(vec, iter);
        res_ttype.as_ref()?;
        let res_ttype = FromPrimitive::from_u8(res_ttype.unwrap());
        res_ttype.as_ref()?;
        Option::Some(Plan{
            title: res_title.unwrap(),
            urgency: res_urg.unwrap(),
            ttype: res_ttype.unwrap(),
        })
    }
}

impl conz::PrettyPrintable for Plan{
    type ArgType = bool;
    fn pretty_print(&self, print_type: &Self::ArgType) -> (astr::AstrVec,Vec<conz::MsgType>){
        let mut text = Vec::new();
        let mut types = Vec::new();
        text.push(self.title.clone());
        text.push(self.urgency.to_string().to_astr());
        types.push(conz::MsgType::Normal);
        types.push(conz::MsgType::Value);
        if *print_type{
            text.push(self.ttype.to_astr());
            types.push(conz::MsgType::Normal);
        }
        (text,types)
    }

    fn lengths(print_type: &Self::ArgType) -> Vec<u16>{
        if !*print_type {vec![48,8]}
        else {vec![48,8,8]}
    }

    fn titles(print_type: &Self::ArgType) -> Vec<astr::Astr>{
        let mut res = vec![astr::from_str("Title:"),
            astr::from_str("Urgency:")];
        if *print_type {
            res.push(astr::from_str("Type:"));
        }
        res
    }
}

impl std::cmp::Ord for Plan {
    fn cmp(&self, other: &Plan) -> std::cmp::Ordering {
        let ontype = self.ttype.cmp(&other.ttype);
        if ontype != std::cmp::Ordering::Equal{
            return ontype;
        }
        self.urgency.cmp(&other.urgency)
    }
}

impl std::cmp::PartialOrd for Plan {
    fn partial_cmp(&self, other: &Plan) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Plan {
    fn eq(&self, other: &Plan) -> bool {
        self.title == other.title &&
        self.urgency == other.urgency &&
        self.ttype == other.ttype
    }
}

impl wizard::Wizardable for Plan{
    fn extract(wres: &mut wizard::WizardRes) -> Option<Plan>{
        loop{
            let title_res = wres.get_text();
            if title_res.is_none() {break;}
            let urgency = wres.get_u16();
            if urgency.is_none() {break;}
            let ttype = wres.get_text();
            if ttype.is_none() {break;}
            let ret = Plan::new(title_res.unwrap(), urgency.unwrap(), ttype.unwrap());
            return Option::Some(ret);
        }
        conz::println_type("Error: could not build todo.", conz::MsgType::Error);
        Option::None
    }

    fn get_fields(partial: bool) -> wizard::FieldVec{
        let mut fields = wizard::FieldVec::new();
        if partial{
            fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Partial);
            fields.add(wizard::InputType::U16, astr::from_str("Urgency: "), wizard::PromptType::Partial);
            fields.add(wizard::InputType::Text, astr::from_str("Type: "), wizard::PromptType::Partial);
        }else{
            fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Once);
            fields.add(wizard::InputType::U16, astr::from_str("Urgency: "), wizard::PromptType::Reprompt);
            fields.add(wizard::InputType::Text, astr::from_str("Type: "), wizard::PromptType::Once);
        }
        fields
    }

    fn get_partial(wres: &mut wizard::WizardRes) -> Self{
        let ttitle = astr::Astr::unwrap_default(wres.get_text());
        let turgency = u16::unwrap_default(wres.get_u16());
        let x = wres.get_text();
        let ttype = PlanType::from_astr(&astr::Astr::unwrap_default(x), true);
        Plan{
            title: ttitle,
            urgency: turgency,
            ttype,
        }
    }

    fn replace_parts(&mut self, replacements: &Self){
        self.title.replace_if_not_default(replacements.title.clone());
        self.urgency.replace_if_not_default(replacements.urgency);
        self.ttype.replace_if_not_default(replacements.ttype.clone());
    }

    fn score_againts(&self, other: &Self) -> i32{
        let mut curr_score = 0;
        if self.title == other.title{
            curr_score += 1;
        }
        if self.urgency == other.urgency{
            curr_score += 1;
        }
        if self.ttype == other.ttype{
            curr_score += 1;
        }
        curr_score
    }

    fn get_name() -> astr::Astr{
        astr::from_str("plan")
    }
}

impl conz::Printable for Plan{
    fn print(&self){
        conz::print_type("Title: ", conz::MsgType::Normal);
        conz::println_type(self.title.disp(), conz::MsgType::Highlight);
        conz::print_type("Urgency: ", conz::MsgType::Normal);
        conz::println_type(format!("{}", self.urgency), conz::MsgType::Highlight);
        conz::print_type("Type: ", conz::MsgType::Normal);
        conz::println_type(self.ttype.to_astr().disp(), conz::MsgType::Highlight);
    }
}

#[derive(FromPrimitive,ToPrimitive,Eq,Clone)]
pub enum SliceType{
    None = 0,
    Deadline = 1,
    Goto = 2,
    Activity = 3,
    DefaultValue = 255,
}

impl PartialEq for SliceType {
    fn eq(&self, other: &Self) -> bool {
        ToPrimitive::to_u8(self) == ToPrimitive::to_u8(other)
    }
}

impl SliceType {
    pub fn from_astr(string: &astr::Astr, partial: bool) -> Self{
        let string = string.to_lower();
        if string.is_empty() && partial{
            return SliceType::DefaultValue;
        }
        if string.cut(1) == astr::from_str("d"){
            return SliceType::Deadline;
        }
        if string.cut(1) == astr::from_str("g"){
            return SliceType::Goto;
        }
        if string.cut(1) == astr::from_str("a"){
            return SliceType::Activity;
        }
        SliceType::None
    }
}

impl astr::ToAstr for SliceType {
    fn to_astr(&self) -> astr::Astr{
        astr::from_str(match self{
            SliceType::None => "None",
            SliceType::Deadline => "Deadline",
            SliceType::Goto => "Goto",
            SliceType::Activity => "Activity",
            SliceType::DefaultValue => "Error",
        })
    }
}

impl DefaultValue for SliceType {
    fn default_val() -> Self{
        SliceType::DefaultValue
    }
}

#[derive(Eq, Clone)]
pub struct Slice {
    pub start: DT,
    pub end: DT,
    title: astr::Astr,
    stype: SliceType,
}

impl Slice {
    pub fn from(start: DT, end: DT, title: astr::Astr, stype: SliceType) -> Self{
        Self{
            start,
            end,
            title,
            stype,
        }
    }
}

impl save::Bufferable for Slice {
    fn into_buffer(&self, vec: &mut Vec<u8>){
        self.start.into_buffer(vec);
        self.end.into_buffer(vec);
        self.title.into_buffer(vec);
        let primtype = ToPrimitive::to_u8(&self.stype);
        if let Some(primtypev) = primtype{
            primtypev.into_buffer(vec);
        }else{
            conz::println_type("Error: Could not convert SliceType to u8.", conz::MsgType::Error);
            (0 as u8).into_buffer(vec);
        }
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Option<Self>{
        if (vec.len() as i32) - (*iter as i32) < 18 { return Option::None; }
        let start = DT::from_buffer(vec, iter);
        start.as_ref()?;
        let end = DT::from_buffer(vec, iter);
        end.as_ref()?;
        let title = astr::Astr::from_buffer(vec, iter);
        title.as_ref()?;
        let res_stype = u8::from_buffer(vec, iter);
        res_stype.as_ref()?;
        let res_stype = FromPrimitive::from_u8(res_stype.unwrap());
        res_stype.as_ref()?;
        Option::Some(Self::from(start.unwrap(), end.unwrap(), title.unwrap(), res_stype.unwrap()))
    }
}

impl std::cmp::Ord for Slice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl std::cmp::PartialOrd for Slice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Slice {
    fn eq(&self, other: &Self) -> bool {
            self.start == other.start &&
            self.end == other.end
    }
}

impl DefaultValue for Slice{
    fn default_val() -> Self{
        let defdt = DT::make_datetime((1,1,1900), (0,0,0)).expect("Expect: DefaultValue for DT");
        Self::from(defdt.clone(), defdt, astr::new(), SliceType::DefaultValue)
    }
}

impl conz::Printable for Slice{
    fn print(&self){
        conz::print_type("Title: ", conz::MsgType::Normal);
        conz::println_type(self.title.disp(), conz::MsgType::Highlight);
        conz::print_type("Type: ", conz::MsgType::Normal);
        conz::println_type(self.stype.to_astr().disp(), conz::MsgType::Highlight);
        conz::print_type("Start: ", conz::MsgType::Normal);
        conz::print_type(self.start.str_datetime().disp(), conz::MsgType::Value);
        tbl::print(" ");
        conz::println_type(self.start.str_dayname().disp(), conz::MsgType::Value);
        conz::print_type("End: ", conz::MsgType::Normal);
        conz::print_type(self.end.str_datetime().disp(), conz::MsgType::Value);
        tbl::print(" ");
        conz::println_type(self.end.str_dayname().disp(), conz::MsgType::Value);
    }
}

impl conz::PrettyPrintable for Slice{
    type ArgType = u8;
    fn pretty_print(&self, _: &Self::ArgType) -> (astr::AstrVec,Vec<conz::MsgType>){
        let mut text = Vec::new();
        let mut types = Vec::new();
        text.push(self.title.clone());
        text.push(self.start.str_datetime()
            .concat(astr::from_str(" "))
            .concat(self.start.str_dayname_short()));
        text.push(self.end.str_datetime()
            .concat(astr::from_str(" "))
            .concat(self.end.str_dayname_short()));
        text.push(self.stype.to_astr());
        types.push(conz::MsgType::Normal);
        types.push(conz::MsgType::Value);
        types.push(conz::MsgType::Value);
        types.push(conz::MsgType::Normal);
        (text,types)
    }

    fn lengths(_: &Self::ArgType) -> Vec<u16>{
        vec![32,23,23,11]
    }

    fn titles(_: &Self::ArgType) -> Vec<astr::Astr>{
        vec![astr::from_str("Title:"),
            astr::from_str("Start Time Date:"),
            astr::from_str("End Time Date:"),
            astr::from_str("Type:"),]
    }
}

impl wizard::Wizardable for Slice{
    fn extract(wres: &mut wizard::WizardRes) -> Option<Self>{
        loop{
            let start_res = wres.get_dt();
            if start_res.is_none() {break;}
            let end_res = wres.get_dt();
            if end_res.is_none() {break;}
            let title_res = wres.get_text();
            if title_res.is_none() {break;}
            let stype_res = wres.get_text();
            if stype_res.is_none() {break;}
            let ret = Slice::from(start_res.unwrap(), end_res.unwrap(), title_res.unwrap(),
                SliceType::from_astr(&stype_res.unwrap(), false));
            return Option::Some(ret);
        }
        conz::println_type("Error: could not build slice.", conz::MsgType::Error);
        Option::None
    }

    fn get_fields(partial: bool) -> wizard::FieldVec{
        let mut fields = wizard::FieldVec::new();
        if partial{
            fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Partial);
            fields.add(wizard::InputType::Text, astr::from_str("Type: "), wizard::PromptType::Partial);
            fields.add(wizard::InputType::DateTime, astr::from_str("Start time date: "), wizard::PromptType::Partial);
            fields.add(wizard::InputType::DateTime, astr::from_str("End time date: "), wizard::PromptType::Partial);
        }else{
            fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Once);
            fields.add(wizard::InputType::Text, astr::from_str("Type: "), wizard::PromptType::Once);
            fields.add(wizard::InputType::DateTime, astr::from_str("Start time date: "), wizard::PromptType::Reprompt);
            fields.add(wizard::InputType::DateTime, astr::from_str("End time date: "), wizard::PromptType::Reprompt);
        }
        fields
    }

    fn get_partial(wres: &mut wizard::WizardRes) -> Self{
        let stitle = astr::Astr::unwrap_default(wres.get_text());
        let stype = SliceType::from_astr(&astr::Astr::unwrap_default(wres.get_text()), true);
        let sstart = DT::unwrap_default(wres.get_dt());
        let send = DT::unwrap_default(wres.get_dt());
        Slice{
            start: sstart,
            end: send,
            title: stitle,
            stype,
        }
    }

    fn replace_parts(&mut self, replacements: &Self){
        self.title.replace_if_not_default(replacements.title.clone());
        self.start.replace_if_not_default(replacements.start.clone());
        self.end.replace_if_not_default(replacements.end.clone());
        self.stype.replace_if_not_default(replacements.stype.clone());
    }

    fn score_againts(&self, other: &Self) -> i32{
        let mut curr_score = 0;
        if self.title == other.title{
            curr_score += 1;
        }
        if self.stype == other.stype{
            curr_score += 1;
        }
        if self.start == other.start{
            curr_score += 1;
        }
        if self.end == other.end{
            curr_score += 1;
        }
        curr_score
    }

    fn get_name() -> astr::Astr{
        astr::from_str("slice")
    }
}

#[derive(Eq, Clone)]
pub struct Todo {
    title: astr::Astr,
    pub done: bool,
}

impl save::Bufferable for Todo {
    fn into_buffer(&self, vec: &mut Vec<u8>){
        self.title.into_buffer(vec);
        (self.done as u8).into_buffer(vec);
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Option<Self>{
        if (vec.len() as i32) - (*iter as i32) < 5 { return Option::None; }
        let title = astr::Astr::from_buffer(vec, iter);
        title.as_ref();
        let done = u8::from_buffer(vec, iter);
        done.as_ref();
        Option::Some(Self{title: title.unwrap(),done: done.unwrap() != 0})
    }
}

impl std::cmp::Ord for Todo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.done.cmp(&other.done)
    }
}

impl std::cmp::PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
            self.done == other.done
    }
}

impl DefaultValue for Todo{
    fn default_val() -> Self{
        Self{title: astr::new(), done: false}
    }
}

impl conz::Printable for Todo{
    fn print(&self){
        conz::print_type("Title: ", conz::MsgType::Normal);
        conz::println_type(self.title.disp(), conz::MsgType::Highlight);
        conz::print_type("Done:", conz::MsgType::Normal);
        conz::println_type(format!("{}", self.done), conz::MsgType::Highlight);
    }
}

impl conz::PrettyPrintable for Todo{
    type ArgType = u8;
    fn pretty_print(&self, _: &Self::ArgType) -> (astr::AstrVec,Vec<conz::MsgType>){
        fn bool_tickbox(b: bool) -> astr::Astr{
            if b {astr::from_str("[ * ]")}
            else {astr::from_str("[   ]")}
        }
        let mut text = Vec::new();
        let mut types = Vec::new();
        text.push(bool_tickbox(self.done));
        text.push(self.title.clone());
        types.push(conz::MsgType::Value);
        types.push(conz::MsgType::Normal);
        (text,types)
    }

    fn lengths(_: &Self::ArgType) -> Vec<u16>{
        vec![5,48]
    }

    fn titles(_: &Self::ArgType) -> Vec<astr::Astr>{
        vec![astr::from_str("Done:"),
            astr::from_str("Title:"),]
    }
}

impl wizard::Wizardable for Todo{
    fn extract(wres: &mut wizard::WizardRes) -> Option<Self>{
        loop{
            let title_res = wres.get_text();
            if title_res.is_none() {break;}
            let done_res = wres.get_bool();
            if done_res.is_none() {break;}
            return Option::Some(Self{title:title_res.unwrap(),done:done_res.unwrap()});
        }
        conz::println_type("Error: could not build todo.", conz::MsgType::Error);
        Option::None
    }

    fn get_fields(partial: bool) -> wizard::FieldVec{
        let mut fields = wizard::FieldVec::new();
        if partial{
            fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Partial);
            fields.add(wizard::InputType::Bool, astr::from_str("Done: "), wizard::PromptType::Partial);
        }else{
            fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Once);
            fields.add(wizard::InputType::Bool, astr::from_str("Done: "), wizard::PromptType::Reprompt);
        }
        fields
    }

    fn get_partial(wres: &mut wizard::WizardRes) -> Self{
        let ttitle = astr::Astr::unwrap_default(wres.get_text());
        let tdone = bool::unwrap_default(wres.get_bool());
        Todo{
            title: ttitle,
            done: tdone,
        }
    }

    fn replace_parts(&mut self, replacements: &Self){
        self.title.replace_if_not_default(replacements.title.clone());
        self.done.replace_if_not_default(replacements.done);
    }

    fn score_againts(&self, other: &Self) -> i32{
        let mut curr_score = 0;
        if self.title == other.title{
            curr_score += 1;
        }
        if self.done == other.done{
            curr_score += 1;
        }
        curr_score
    }

    fn get_name() -> astr::Astr{
        astr::from_str("todo")
    }
}
