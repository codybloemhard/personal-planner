use super::conz;
use super::conz::PrinterFunctions;
use super::conz::Printable;
use super::data;
use super::astr;
use super::astr::AStr;
use super::astr::ToAstr;
use super::wizard;
use super::state;
use super::misc::{UnwrapDefault, DefaultValue};

pub fn now(_: &mut state::State, _: astr::AstrVec){
    let dt = data::DT::new();
    conz::printer().print_type(&dt.str_datetime(), conz::MsgType::Value);
    conz::printer().print(&" ");
    conz::printer().println_type(&dt.str_dayname(), conz::MsgType::Value);
}

pub fn mk_point(state: &mut state::State, _: astr::AstrVec){
    conz::printer().println_type(&"Add deadline: ", conz::MsgType::Normal);
    let mut fields = wizard::FieldVec::new();
    fields.add(wizard::InputType::Text, astr::from_str("title: "), false);
    fields.add(wizard::InputType::Text, astr::from_str("type: "), false);
    fields.add(wizard::InputType::DateTime, astr::from_str("time date: "), true);
    let res = fields.execute();
    if res.is_err() {return;}
    let mut res = res.unwrap();
    let point = res.extract_point();
    if point.is_err() {return;}
    state.points.add_item(point.unwrap());
    if !state.points.write() {return;}
    conz::printer().println_type(&"Success: Point saved.", conz::MsgType::Highlight);
}

pub fn rm_point(state: &mut state::State, _: astr::AstrVec){
    conz::printer().println_type(&"Remove point(search first): ", conz::MsgType::Normal);
    let mut fields = wizard::FieldVec::new();
    fields.add(wizard::InputType::Text, astr::from_str("title: "), false);
    fields.add(wizard::InputType::Text, astr::from_str("type: "), false);
    fields.add(wizard::InputType::Text, astr::from_str("time date: "), true);
    loop{
        let res = fields.execute();
        if res.is_err() {return;}
        let mut res = res.unwrap();
        let ptitle = astr::Astr::unwrap_default(res.get_text());
        let ptype = data::PointType::from_astr(&astr::Astr::unwrap_default(res.get_text()));
        let pdt = data::DT::unwrap_default(res.get_dt());
        let mut score = 0;
        let mut more_than_one = false;
        let mut vec = Vec::new();
        let points = state.points.get_items();
        for i in 0..points.len(){
            let current = &points[i];
            let mut curr_score = 0;
            if ptitle == current.title{
                curr_score += 1;
            }
            if ptype == current.ptype{
                curr_score += 1;
            }
            if pdt == current.dt{
                curr_score += 1;
            }
            if curr_score > score{
                score = curr_score;
                more_than_one = false;
                vec.clear();
                vec.push(i);
            }
            else if curr_score == score{
                more_than_one = true;
                vec.push(i);
            }
        }
        if score > 0 && !more_than_one{
            conz::printer().println_type(&"Success: found a match:", conz::MsgType::Highlight);
            points[vec[0]].print();
            match conz::read_bool(&"Delete this item?: "){
                true =>{
                    let ok = state.points.remove_indices(vec);
                    if ok {
                        conz::printer().println_type(&"Success: Item removed.", conz::MsgType::Highlight);
                    }else{
                        conz::printer().println_type(&"Error: Item removing failed.", conz::MsgType::Highlight);
                    }
                    return;
                }
                false =>{return;}
            }
        }
        if score == 0{
            conz::printer().println_type(&"Fail: no matches found.", conz::MsgType::Error);
            match conz::read_bool(&"Try again?: "){
                true =>{continue;}
                false =>{return;}
            }
        }
        if more_than_one{
            conz::printer().println_type(&"Warning: query is ambiguous.", conz::MsgType::Error);
            conz::printer().print_type(&"Found ", conz::MsgType::Normal);
            conz::printer().print_type(&format!("{}", vec.len()), conz::MsgType::Value);
            conz::printer().println_type(&" items.", conz::MsgType::Normal);
            for i in &vec{
                points[*i].print();
            }
            match conz::read_bool(&"Delete all?: "){
                true =>{
                    let ok = state.points.remove_indices(vec);
                    if ok {
                        conz::printer().println_type(&"Success: Items removed.", conz::MsgType::Highlight);
                    }else{
                        conz::printer().println_type(&"Error: Items removing failed.", conz::MsgType::Highlight);
                    }
                    return;
                }
                false =>{}
            }
            match conz::read_bool(&"Try again?: "){
                true =>{continue;}
                false =>{return;}
            }
        }
    }
}

pub fn edit_point(state: &mut state::State, _: astr::AstrVec){
    conz::printer().println_type(&"Edit point(search first): ", conz::MsgType::Normal);
    let mut fields = wizard::FieldVec::new();
    fields.add(wizard::InputType::Text, astr::from_str("title: "), false);
    fields.add(wizard::InputType::Text, astr::from_str("type: "), false);
    fields.add(wizard::InputType::Text, astr::from_str("time date: "), true);
    loop{
        let res = fields.execute();
        if res.is_err() {return;}
        let mut res = res.unwrap();
        let ptitle = astr::Astr::unwrap_default(res.get_text());
        let ptype = data::PointType::from_astr(&astr::Astr::unwrap_default(res.get_text()));
        let pdt = data::DT::unwrap_default(res.get_dt());
        let mut score = 0;
        let mut more_than_one = false;
        let mut on_ptitle = false;
        let mut on_ptype = false;
        let mut on_pdt = false;
        let mut vec = Vec::new();
        let points = state.points.get_items();
        for i in 0..points.len(){
            let current = &points[i];
            let mut curr_score = 0;
            let mut lon_ptitle = false;
            let mut lon_ptype = false;
            let mut lon_pdt = false;
            if ptitle == current.title{
                curr_score += 1;
                lon_ptitle = true;
            }
            if ptype == current.ptype{
                curr_score += 1;
                lon_ptype = true;
            }
            if pdt == current.dt{
                curr_score += 1;
                lon_pdt = true;
            }
            if curr_score > score{
                score = curr_score;
                more_than_one = false;
                on_ptitle = false;
                on_ptype = false;
                on_pdt = false;
                vec.clear();
                vec.push(i);
            }
            else if curr_score == score{
                more_than_one = true;
                on_ptitle = lon_ptitle;
                on_ptype = lon_ptype;
                on_pdt = lon_pdt;
                vec.push(i);
            }
        }
        if score > 0 && !more_than_one{
            conz::printer().println_type(&"Success: found a match:", conz::MsgType::Highlight);
            points[vec[0]].print();
            match conz::read_bool(&"Edit this item?: "){
                true =>{
                    let res = fields.execute();
                    if res.is_err() {return;}
                    let mut res = res.unwrap();
                    let nptitle = astr::Astr::unwrap_default(res.get_text());
                    let nptype = data::PointType::from_astr(&astr::Astr::unwrap_default(res.get_text()));
                    let npdt = data::DT::unwrap_default(res.get_dt());
                    let mut npoint = points[vec[0]].clone();
                    npoint.title.replace_if_not_default(nptitle);
                    npoint.ptype.replace_if_not_default(nptype);
                    npoint.dt.replace_if_not_default(npdt);
                    let ok = state.points.replace_indices(vec, npoint);
                    if ok {
                        conz::printer().println_type(&"Success: Item edited.", conz::MsgType::Highlight);
                    }else{
                        conz::printer().println_type(&"Error: Item editing failed.", conz::MsgType::Highlight);
                    }
                    return;
                }
                false =>{return;}
            }
        }
        if score == 0{
            conz::printer().println_type(&"Fail: no matches found.", conz::MsgType::Error);
            match conz::read_bool(&"Try again?: "){
                true =>{continue;}
                false =>{return;}
            }
        }
        if more_than_one{
            conz::printer().println_type(&"Warning: not implemented.", conz::MsgType::Error);
            /*conz::printer().println_type(&"Warning: query is ambiguous.", conz::MsgType::Error);
            conz::printer().print_type(&"Found ", conz::MsgType::Normal);
            conz::printer().print_type(&format!("{}", vec.len()), conz::MsgType::Value);
            conz::printer().println_type(&" items.", conz::MsgType::Normal);
            for i in &vec{
                points[*i].print();
            }
            match conz::read_bool(&"Edit(replace with one new) all?: "){
                true =>{
                    let mut nfields = wizard::FieldVec::new();
                    if on_ptitle{
                        nfields.add(wizard::InputType::Text, astr::from_str("title: "), false);
                    }
                    if on_ptype{
                        nfields.add(wizard::InputType::Text, astr::from_str("type: "), false);
                    }
                    if on_pdt{
                        nfields.add(wizard::InputType::Text, astr::from_str("time date: "), true);
                    }
                    let res = nfields.execute();
                    if res.is_err() {return;}
                    let mut res = res.unwrap();
                    let nptitle = if on_ptitle{astr::Astr::unwrap_default(res.get_text())}
                        else {astr::Astr::default_val()};
                    let nptype = if on_ptype{data::PointType::from_astr(&astr::Astr::unwrap_default(res.get_text()))}
                        else{data::PointType::default_val()};
                    let npdt = if on_pdt{data::DT::unwrap_default(res.get_dt())}
                        else{data::DT::default_val()};
                    let mut npoint = points[vec[0]].clone();
                    npoint.title.replace_if_not_default(nptitle);
                    npoint.ptype.replace_if_not_default(nptype);
                    npoint.dt.replace_if_not_default(npdt);
                    let ok = state.points.replace_indices(vec, npoint);
                    if ok {
                        conz::printer().println_type(&"Success: Items edited.", conz::MsgType::Highlight);
                    }else{
                        conz::printer().println_type(&"Error: Items editing failed.", conz::MsgType::Highlight);
                    }
                    return;
                }
                false =>{}
            }
            match conz::read_bool(&"Try again?: "){
                true =>{continue;}
                false =>{return;}
            }*/
        }
    }
}

pub fn ls_points(state: &mut state::State, _: astr::AstrVec){
    let count = state.points.get_items().len();
    let len_title = 32; let len_relative = 14; let len_dt = 23; let len_type = 11;
    conz::printer().print_type(&"Found ", conz::MsgType::Normal);
    conz::printer().print_type(&format!("{}", count), conz::MsgType::Value);
    conz::printer().println_type(&" points.", conz::MsgType::Normal);
    let divider_ver = || {conz::printer().print_type(&" | ", conz::MsgType::Highlight);};
    let divider_ver_edge = || {conz::printer().print_type(&"|", conz::MsgType::Highlight);};
    let divider_hor = |a| {astr::from_str("|")
        .concat(astr::from_str(a).repeat(len_title + len_relative + len_dt + len_type + (3*3)))
        .concat(astr::from_str("|"))};
    conz::printer().println_type(&divider_hor("="), conz::MsgType::Highlight);
    divider_ver_edge();
    conz::printer().print_type(
        &astr::from_str("title:").pad_after(len_title), 
        conz::MsgType::Normal);
    divider_ver();
    conz::printer().print_type(
        &astr::from_str("relative:").pad_after(len_relative), 
        conz::MsgType::Normal);
    divider_ver();
    conz::printer().print_type(
        &astr::from_str("time date:").pad_after(len_dt),
        conz::MsgType::Normal);
        divider_ver();
    conz::printer().print_type(
        &astr::from_str("type:").pad_after(len_type),
        conz::MsgType::Normal);
    divider_ver_edge();
    conz::printer().println(&"");
    conz::printer().println_type(&divider_hor("-"), conz::MsgType::Highlight);
    let now = data::DT::new();
    for x in state.points.get_items(){
        let diff = now.diff(&x.dt);
        let timecol = if diff.neg{
            conz::MsgType::Error
        }else if diff.total_hours <= 48 {
            conz::MsgType::Highlight
        }else{
            conz::MsgType::Normal
        };
        divider_ver_edge();
        conz::printer().print_type(
            &x.title.pad_after(len_title),
            conz::MsgType::Normal);
        divider_ver();
        conz::printer().print_type(
            &diff.string_significant()
                .to_astr()
                .pad_after(len_relative),
            timecol);
        divider_ver();
        conz::printer().print_type(
            &x.dt.str_datetime().concat(astr::from_str(" "))
                .concat(x.dt.str_dayname_short()).pad_after(len_dt),
            conz::MsgType::Value);
        divider_ver();
        conz::printer().print_type(
            &x.ptype.to_astr().pad_after(len_type),
            conz::MsgType::Normal);
        divider_ver_edge();
        conz::printer().println(&"");
    }
    conz::printer().println_type(&divider_hor("="), conz::MsgType::Highlight);
}

pub fn flush_files(state: &mut state::State, _: astr::AstrVec){
    if state.is_clean() {
        conz::printer().println_type(&"All files clean, nothing to do.", conz::MsgType::Highlight);
        return;
    }
    let res = state.flush_files();
    if res {
        conz::printer().println_type(&"Success: Flushed all dirty files.", conz::MsgType::Highlight);
    }else{
        conz::printer().println_type(&"Error: Could not flush all dirty files.", conz::MsgType::Error);
    }
}
