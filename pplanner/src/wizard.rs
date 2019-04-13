use std::collections::VecDeque;

use super::astr;
use super::data;
use super::conz;
use super::astr::*;

pub enum InputType{
    Text,
    DateTime,
}

struct Field{
    field_type: InputType,
    prompt_msg: astr::Astr,
    reprompt: bool,
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

    pub fn add(&mut self, field_type: InputType, prompt_msg: astr::Astr, reprompt: bool){
        self.vec.push(Field{
            field_type: field_type,
            prompt_msg: prompt_msg,
            reprompt: reprompt,
        });
    }

    pub fn execute(&self, printer: &mut conz::Printer) -> Result<WizardRes,()>{
        let mut texts: VecDeque<astr::Astr> = VecDeque::new();
        let mut datetimes: VecDeque<data::DT> = VecDeque::new();
        for instr in &self.vec{
            loop {
                let is_ok = match instr.field_type{
                    InputType::Text => Self::handle_text(&mut texts, &instr, printer),
                    InputType::DateTime => Self::handle_datetime(&mut datetimes, &instr, printer),
                };
                if is_ok { break; }
                if !instr.reprompt { return Err(()); }
                let redo = conz::prompt(printer, "Could not parse, try again? */n: ");
                if redo == "n" { return Err(()); }
            }
        }
        let res = WizardRes::new(texts, datetimes);
        return Ok(res);
    }

    fn handle_text(texts: &mut VecDeque<astr::Astr>, field: &Field, printer: &mut conz::Printer) -> bool{
        let line = conz::prompt(printer, &field.prompt_msg.to_string()).to_astr();
        texts.push_back(line);
        return true;
    }

    fn handle_datetime(datetimes: &mut VecDeque<data::DT>, field: &Field, printer: &mut conz::Printer) -> bool{
        let lines = astr::from_str(&conz::prompt(printer, &field.prompt_msg.to_string())).split_str(&astr::astr_whitespace());
        if lines.len() != 2 { return false; }
        let tri0 = data::parse_dmy_or_hms(&lines[0]);
        let tri1 = data::parse_dmy_or_hms(&lines[1]);
        if tri0.is_err() { return false; }
        if tri1.is_err() { return false; }
        let dt1 = data::DT::make_datetime(tri1.unwrap(), tri0.unwrap());
        if dt1.is_err() { return false; }
        datetimes.push_back(dt1.unwrap());
        return true;
    }
}

pub struct WizardRes{
    all_text: VecDeque<astr::Astr>,
    all_datetime: VecDeque<data::DT>,
}

impl WizardRes{
    pub fn new(text: VecDeque<astr::Astr>, dt: VecDeque<data::DT>) -> Self{
        WizardRes{
            all_text: text,
            all_datetime: dt,
        }
    }

    pub fn extract_deadline(&mut self) -> Result<data::Deadline,()>{
        if self.all_text.len() < 1 {return Err(());}
        if self.all_datetime.len() < 1 {return Err(());}
        let dt_res = self.all_datetime.pop_front();
        if dt_res.is_none() {return Err(());}
        let title_res = self.all_text.pop_front();
        if title_res.is_none() {return Err(());}
        let ret = data::Deadline::new(dt_res.unwrap(), title_res.unwrap());
        return Ok(ret);
    }
}