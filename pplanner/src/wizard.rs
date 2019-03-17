use super::astr;
use super::data;
use super::conz;
use super::astr::*;

pub enum InputType{
    Text,
    DateTime,
}

pub struct Field{
    field_type: InputType,
    prompt_msg: astr::Astr,
    reprompt: bool,
}

pub struct WizardRes{
    all_text: Vec<astr::Astr>,
    all_datetime: Vec<data::DT>,
}

pub fn execute(instructions: &Vec<Field>, printer: &mut conz::Printer) -> Result<WizardRes,()>{
    let mut texts: Vec<astr::Astr> = Vec::new();
    let mut datetimes: Vec<data::DT> = Vec::new();
    for instr in instructions{
        let is_ok = match instr.field_type{
            InputType::Text => handle_text(&mut texts, &instr, printer),
            InputType::DateTime => handle_datetime(&mut datetimes, &instr, printer),
        };
        if !is_ok{
            return Err(());
        }
    }
    let res = WizardRes{
        all_text: texts,
        all_datetime: datetimes,
    };
    return Ok(res);
}

fn handle_text(texts: &mut Vec<astr::Astr>, field: &Field, printer: &mut conz::Printer) -> bool{
    let line = conz::prompt(printer, &field.prompt_msg.to_string()).to_astr();
    texts.push(line);
    return true;
}

fn handle_datetime(datetimes: &mut Vec<data::DT>, field: &Field, printer: &mut conz::Printer) -> bool{
    return false;
}