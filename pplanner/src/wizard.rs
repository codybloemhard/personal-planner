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

pub fn make_field(field_type: InputType, prompt_msg: astr::Astr, reprompt: bool) -> Field{
    return Field{
        field_type: field_type,
        prompt_msg: prompt_msg,
        reprompt: reprompt,
    }
}

pub fn execute(instructions: &Vec<Field>, printer: &mut conz::Printer) -> Result<WizardRes,()>{
    let mut texts: Vec<astr::Astr> = Vec::new();
    let mut datetimes: Vec<data::DT> = Vec::new();
    for instr in instructions{
        loop {
            let is_ok = match instr.field_type{
                InputType::Text => handle_text(&mut texts, &instr, printer),
                InputType::DateTime => handle_datetime(&mut datetimes, &instr, printer),
            };
            if is_ok { break; }
            if !instr.reprompt { return Err(()); }
            let redo = conz::prompt(printer, "Could not parse, try again? */n: ");
            if redo == "n" { return Err(()); }
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
    let lines = astr::from_str(&conz::prompt(printer, &field.prompt_msg.to_string())).split_str(&astr::astr_whitespace());
    if lines.len() != 2 { return false; }
    let tri0 = data::parse_dmy_or_hms(&lines[0]);
    let tri1 = data::parse_dmy_or_hms(&lines[1]);
    if tri0.is_err() { return false; }
    if tri1.is_err() { return false; }
    let dt1 = data::DT::make_datetime(tri1.unwrap(), tri0.unwrap());
    if dt1.is_err() { return false; }
    datetimes.push(dt1.unwrap());
    return true;
}