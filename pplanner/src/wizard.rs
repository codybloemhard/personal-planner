use std::collections::VecDeque;

use super::astr;
use super::data;
use super::conz;
use super::astr::*;
use super::astr::{AStr};
use super::misc::{DefaultValue};

//use std::time::{SystemTime, UNIX_EPOCH};

pub enum InputType{
    Text,
    DateTime,
    U16,
}

pub enum PromptType{
    Reprompt,   //ask until good or user cancels -> push good val or error
    Once,       //ask one time -> push good val or error
    Partial,    //ask one time -> push good val or push nothing
}

struct Field{
    field_type: InputType,
    prompt_msg: astr::Astr,
    prompt_type: PromptType,
}

pub struct FieldVec{
    vec: Vec<Field>,
}

impl FieldVec{
    pub fn new() -> Self{
        FieldVec{
            vec: Vec::new(),
        }
    }

    pub fn add(&mut self, field_type: InputType, prompt_msg: astr::Astr, prompt_type: PromptType){
        self.vec.push(Field{
            field_type: field_type,
            prompt_msg: prompt_msg,
            prompt_type: prompt_type,
        });
    }

    pub fn execute(&self, inputs: &mut Option<VecDeque<astr::Astr>>) -> Option<WizardRes>{
        let mut texts: VecDeque<astr::Astr> = VecDeque::new();
        let mut datetimes: VecDeque<data::DT> = VecDeque::new();
        let mut u16s: VecDeque<u16> = VecDeque::new();
        let ask = inputs.is_none();
        for instr in &self.vec{
            loop {
                let line = if ask{
                    conz::prompt(&instr.prompt_msg.to_string()).to_astr()
                }else{
                    let mut res = inputs.as_mut().unwrap().pop_front();
                    if res.is_none(){
                        match instr.prompt_type{
                            PromptType::Partial =>{
                                res = Option::Some(astr::from_str(""));
                            }
                            _ =>{
                                conz::println_type("Error: Not enough inputs provided for command!", 
                                    conz::MsgType::Error);
                                return Option::None;
                            }
                        }
                    }
                    res.unwrap()
                };
                let is_ok = match instr.field_type{
                    InputType::Text => Self::handle_text(&mut texts, line),
                    InputType::DateTime => Self::handle_datetime(&mut datetimes, line),
                    InputType::U16 => Self::handle_u16(&mut u16s, line),
                };
                if is_ok {break;}
                match instr.prompt_type{
                    PromptType::Once =>{
                        conz::println_type("Fail: could not parse.", conz::MsgType::Error);
                        return Option::None;
                    }
                    PromptType::Reprompt =>{
                        let redo = conz::prompt("Could not parse, try again? */n: ");
                        if redo == "n" {return Option::None;}
                    }
                    PromptType::Partial =>{
                        match instr.field_type{
                            InputType::Text => texts.push_back(astr::Astr::default_val()),
                            InputType::DateTime => datetimes.push_back(data::DT::default_val()),
                            InputType::U16 => u16s.push_back(u16::default_val()),
                        }
                        break;
                    }
                }
            }
        }
        let res = WizardRes::new(texts, datetimes, u16s);
        return Option::Some(res);
    }

    fn handle_text(texts: &mut VecDeque<astr::Astr>, line: astr::Astr) -> bool{
        //check if freeze is in stdin
        //let start = SystemTime::now();
        if line.len() < 1 {return false;}
        //let end = SystemTime::now();
        //let dur = end.duration_since(start);
        //println!("{:?}", dur);
        texts.push_back(line);
        return true;
    }

    fn handle_datetime(datetimes: &mut VecDeque<data::DT>, line: astr::Astr) -> bool{
        let lines = line.split_str(&astr::astr_whitespace());
        if lines.len() != 2 {return false;}
        let tri0 = data::parse_hms(&lines[0]);
        let tri1 = data::parse_dmy(&lines[1]);
        if tri0.is_none() {return false;}
        if tri1.is_none() {return false;}
        let dt1 = data::DT::make_datetime(tri1.unwrap(), tri0.unwrap());
        if dt1.is_none() {return false;}
        datetimes.push_back(dt1.unwrap());
        return true;
    }

    fn handle_u16(u16s: &mut VecDeque<u16>, line: astr::Astr) -> bool{
        let val: Option<u16> = term_basics_linux::string_to_value(&line.to_string());
        if val.is_none() {return false;}
        u16s.push_back(val.unwrap());
        return true;
    }
}

pub struct WizardRes{
    all_text: VecDeque<astr::Astr>,
    all_datetime: VecDeque<data::DT>,
    all_u16s: VecDeque<u16>,
}

impl WizardRes{
    pub fn new(text: VecDeque<astr::Astr>, dt: VecDeque<data::DT>, u16s: VecDeque<u16>) -> Self{
        WizardRes{
            all_text: text,
            all_datetime: dt,
            all_u16s: u16s,
        }
    }

    pub fn get_text(&mut self) -> Option<astr::Astr>{
        let res = self.all_text.pop_front();
        if res.is_none() {return Option::None}
        return Option::Some(res.unwrap());
    }

    pub fn get_dt(&mut self) -> Option<data::DT>{
        let res = self.all_datetime.pop_front();
        if res.is_none() {return Option::None}
        return Option::Some(res.unwrap());
    }

    pub fn get_u16(&mut self) -> Option<u16>{
        let res = self.all_u16s.pop_front();
        if res.is_none() {return Option::None;}
        return Option::Some(res.unwrap());
    }
}

pub trait Wizardable where Self: std::marker::Sized + conz::Printable{
    fn get_fields(partial: bool) -> FieldVec;
    fn extract(wres: &mut WizardRes) -> Option<Self>;
    fn get_partial(wres: &mut WizardRes) -> Self;
    fn replace_parts(&mut self, replacements: &Self);
    fn score_againts(&self, other: &Self) -> i32;
    fn get_name() -> astr::Astr;
}
