use chrono::prelude::*;
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use term_basics_linux::tbl;

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
        let mut prefix = match self.neg{
            true => "past ",
            false => "in ",
        };
        if as_dur {
            prefix = "";
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
        return format!("{}{} secs", prefix, self.total_secs);
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
        return Option::Some(DT{ dt: datetime.unwrap(), });
    }

    pub fn str_datetime(&self) -> astr::Astr{    
        return format!("{}", self.dt.format("%H:%M:%S %d-%m-%Y")).to_astr();
    }

    pub fn str_date(&self) -> astr::Astr{    
        return format!("{}", self.dt.format("%d-%m-%Y")).to_astr();
    }

    pub fn str_time(&self) -> astr::Astr{    
        return format!("{}", self.dt.format("%H:%M:%S")).to_astr();
    }

    pub fn weeknr(&self) -> u8{
        let datetime = Self::make_datetime((1, 1, self.dt.year() as u32), (0, 0, 0));//first day this year
        if datetime.is_none() { return 0; }
        let datetime = datetime.unwrap();
        let since = self.diff(&datetime);
        let weeks = since.days / 7;
        return (weeks + 1) as u8; //year starts with week 1 not week 0
    }

    pub fn str_weeknr(&self) -> astr::Astr{
        return format!("{}", self.weeknr()).to_astr();
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
        return PointType::DefaultValue;
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
            conz::println_type("Error: Could not convert PointType to u8.", conz::MsgType::Error);
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
        return (text,types);
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
        return Option::None;
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
        return fields;
    }

    fn get_partial(wres: &mut wizard::WizardRes) -> Self{
        let ptitle = astr::Astr::unwrap_default(wres.get_text());
        let ptype = PointType::from_astr(&astr::Astr::unwrap_default(wres.get_text()), true);
        let pdt = DT::unwrap_default(wres.get_dt());
        return Point{
            dt: pdt,
            title: ptitle,
            ptype: ptype,
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
        return curr_score;
    }

    fn get_name() -> astr::Astr{
        astr::from_str("point")
    }
}

#[derive(FromPrimitive,ToPrimitive,Eq,Clone)]
pub enum TodoType{
    Todo,
    Long,
    Idea,
    DefaultValue,
}

impl PartialEq for TodoType {
    fn eq(&self, other: &TodoType) -> bool {
        ToPrimitive::to_u8(self) == ToPrimitive::to_u8(other)
    }
}

impl TodoType{
    pub fn from_astr(string: &astr::Astr, partial: bool) -> TodoType{
        let string = string.to_lower();
        if string.len() == 0 && partial{
            return TodoType::DefaultValue;
        }
        if string[0] == 'l' as u8{
            return TodoType::Long;
        }
        if string[0] == 'i' as u8{
            return TodoType::Idea;
        }
        return TodoType::Todo;
    }
}

impl astr::ToAstr for TodoType{
    fn to_astr(&self) -> astr::Astr{
        astr::from_str(match self{
            TodoType::Todo => "Todo",
            TodoType::Long => "Longterm",
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

impl std::cmp::PartialOrd for TodoType{
    fn partial_cmp(&self, other: &TodoType) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for TodoType{
    fn cmp(&self, other: &TodoType) -> std::cmp::Ordering {
        ToPrimitive::to_u8(self).cmp(&ToPrimitive::to_u8(other))
    }
}

#[derive(Eq,Clone)]
pub struct Todo{
    title: astr::Astr,
    urgency: u16,
    pub ttype: TodoType,
}

impl Todo{
    pub fn new(title: astr::Astr, urgency: u16, strtype: astr::Astr) -> Todo{
        Todo{
            title: title,
            urgency: urgency,
            ttype: TodoType::from_astr(&strtype, false),
        }
    }
}

impl save::Bufferable for Todo{
    fn into_buffer(&self, vec: &mut Vec<u8>){
        self.title.into_buffer(vec);
        self.urgency.into_buffer(vec);
        let primtype = ToPrimitive::to_u8(&self.ttype);
        if primtype.is_none() {
            conz::println_type("Error: Could not convert TodoType to u8.", conz::MsgType::Error);
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

impl conz::PrettyPrintable for Todo{
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
        return (text,types);
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
        return res;
    }
}

impl std::cmp::Ord for Todo {
    fn cmp(&self, other: &Todo) -> std::cmp::Ordering {
        let ontype = self.ttype.cmp(&other.ttype);
        if ontype != std::cmp::Ordering::Equal{
            return ontype;
        }
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
        self.urgency == other.urgency &&
        self.ttype == other.ttype
    }
}

impl wizard::Wizardable for Todo{
    fn extract(wres: &mut wizard::WizardRes) -> Option<Todo>{
        loop{
            let title_res = wres.get_text();
            if title_res.is_none() {break;}
            let urgency = wres.get_u16();
            if urgency.is_none() {break;}
            let ttype = wres.get_text();
            if ttype.is_none() {break;}
            let ret = Todo::new(title_res.unwrap(), urgency.unwrap(), ttype.unwrap());
            return Option::Some(ret);
        }
        conz::println_type("Error: could not build todo.", conz::MsgType::Error);
        return Option::None;
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
        return fields;
    }

    fn get_partial(wres: &mut wizard::WizardRes) -> Self{
        let ttitle = astr::Astr::unwrap_default(wres.get_text());
        let turgency = u16::unwrap_default(wres.get_u16());
        let x = wres.get_text();
        let ttype = TodoType::from_astr(&astr::Astr::unwrap_default(x), true);
        return Todo{
            title: ttitle,
            urgency: turgency,
            ttype: ttype,
        }
    }

    fn replace_parts(&mut self, replacements: &Self){
        self.title.replace_if_not_default(replacements.title.clone());
        self.urgency.replace_if_not_default(replacements.urgency.clone());
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
        return curr_score;
    }

    fn get_name() -> astr::Astr{
        astr::from_str("todo")
    }
}

impl conz::Printable for Todo{
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
        if string.len() == 0 && partial{
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
        return SliceType::None;
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
        return SliceType::DefaultValue;
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
            start: start,
            end: end,
            title: title,
            stype: stype,
        }
    }
}

impl save::Bufferable for Slice {
    fn into_buffer(&self, vec: &mut Vec<u8>){
        self.start.into_buffer(vec);
        self.end.into_buffer(vec);
        self.title.into_buffer(vec);
        let primtype = ToPrimitive::to_u8(&self.stype);
        if primtype.is_none() {
            conz::println_type("Error: Could not convert SliceType to u8.", conz::MsgType::Error);
            (0 as u8).into_buffer(vec);
        }else{
            primtype.unwrap().into_buffer(vec);
        }
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Option<Self>{
        if (vec.len() as i32) - (*iter as i32) < 18 { return Option::None; }
        let start = DT::from_buffer(vec, iter);
        if start.is_none() { return Option::None; }
        let end = DT::from_buffer(vec, iter);
        if end.is_none() { return Option::None; }
        let title = astr::Astr::from_buffer(vec, iter);
        if title.is_none() {return Option::None;}
        let res_stype = u8::from_buffer(vec, iter);
        if res_stype.is_none() {return Option::None;}
        let res_stype = FromPrimitive::from_u8(res_stype.unwrap());
        if res_stype.is_none() {return Option::None;}
        return Option::Some(Self::from(start.unwrap(), end.unwrap(), title.unwrap(), res_stype.unwrap()));
    }
}

impl std::cmp::Ord for Slice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.start.cmp(&other.start);
    }
}

impl std::cmp::PartialOrd for Slice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl std::cmp::PartialEq for Slice {
    fn eq(&self, other: &Self) -> bool {
        return 
            self.start == other.start &&
            self.end == other.end;
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
        return (text,types);
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
        return Option::None;
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
        return fields;
    }

    fn get_partial(wres: &mut wizard::WizardRes) -> Self{
        let stitle = astr::Astr::unwrap_default(wres.get_text());
        let stype = SliceType::from_astr(&astr::Astr::unwrap_default(wres.get_text()), true);
        let sstart = DT::unwrap_default(wres.get_dt());
        let send = DT::unwrap_default(wres.get_dt());
        return Slice{
            start: sstart,
            end: send,
            title: stitle,
            stype: stype,
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
        return curr_score;
    }

    fn get_name() -> astr::Astr{
        astr::from_str("slice")
    }
}
