use std::io::prelude::*;
use std::fs::OpenOptions;

use super::conz;
use super::data;

pub const DATA_DIR: &'static str = ".config/pplanner";
pub const DEADLINE_DIR: &'static str = "deadlines";

pub fn get_data_dir_path(relative: &str) -> Option<std::path::PathBuf>{
    let hd = dirs::home_dir();
    if hd.is_none() {return Option::None;}
    let mut hd = hd.unwrap();
    hd.push(DATA_DIR);
    hd.push(relative);
    return Option::Some(hd);
}

pub fn setup_config_dir() -> bool{
    let mut printer = conz::printer();
    let home = get_data_dir_path("");
    if home.is_none() {
        printer.print_type("Error: could not get home directory.", conz::MsgType::Error);
        return false;
    }
    let path = home.unwrap();
    let path = path.as_path();
    let metatdata = std::fs::metadata(path);
    let pathstr = path.to_str();
    if pathstr.is_none() {
        printer.print_type("Error: could not get string from path.", conz::MsgType::Error);
        return false;
    }
    let pathstr = pathstr.unwrap();
    if !metatdata.is_ok() {
        let res = std::fs::create_dir_all(path);
        if !res.is_ok() {
            printer.print_type("Error: Could not create path: ", conz::MsgType::Error);
            printer.println_type(pathstr, conz::MsgType::Highlight);
            return false;
        }else{
            printer.print_type("First time use: created path: ", conz::MsgType::Normal);
            printer.println_type(pathstr, conz::MsgType::Highlight);
        }
    }
    let dummy: Vec<u8> = Vec::new();
    {
        let deadlinepath = get_data_dir_path(DEADLINE_DIR).unwrap();
        let deadlinepath = deadlinepath.as_path();
        let metatdata = std::fs::metadata(deadlinepath);
        if metatdata.is_err() {
            let ok = buffer_write_file(deadlinepath, &dummy);
            let pathstr = path.to_str();
            if pathstr.is_none() {
                printer.print_type("Error: could not get string from path.", conz::MsgType::Error);
                return false;
            }
            let pathstr = pathstr.unwrap();
            if ok{
                printer.print_type("First time use: created file: ", conz::MsgType::Normal);
                printer.println_type(pathstr, conz::MsgType::Highlight);
            }
            else{
                printer.print_type("Error: Could not create file: ", conz::MsgType::Error);
                printer.println_type(pathstr, conz::MsgType::Highlight);
            }
        }
    }
    return true;
}

pub type Buffer = Vec<u8>;

pub trait Bufferable{
    type Return;
    fn into_buffer(&self, vec: &mut Buffer);
    fn from_buffer(vec: &Buffer, iter: &mut u32) -> Result<Self::Return, ()>;
}

impl Bufferable for u32{
    type Return = u32;
    fn into_buffer(&self, vec: &mut Buffer){
        vec.push(((*self >> 24) & 0xff) as u8);
        vec.push(((*self >> 16) & 0xff) as u8);
        vec.push(((*self >> 8) & 0xff) as u8);
        vec.push((*self & 0xff) as u8);
    }

    fn from_buffer(vec: &Buffer, iter: &mut u32) -> Result<Self::Return, ()>{
        if (vec.len() as i32) - (*iter as i32) < 4 { return Err(()); }
        let mut val: u32 = 0;
        val += (vec[(*iter + 0) as usize] as u32) << 24;
        val += (vec[(*iter + 1) as usize] as u32) << 16;
        val += (vec[(*iter + 2) as usize] as u32) << 8;
        val += vec[(*iter + 3) as usize] as u32;
        *iter += 4;
        return Ok(val);
    }
}

pub fn buffer_append_buffer(vec: &mut Buffer, string: &Buffer){
    for byte in string{
        vec.push(*byte);
    }
}

pub fn buffer_write_file(path: &std::path::Path, vec: &Buffer) -> bool{
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(path);
    if file.is_err() { return false; }
    let mut opened = file.unwrap();
    if opened.write_all(&vec).is_err() { return false; }
    return true;
}

pub fn buffer_read_file(path: &std::path::Path) -> Result<Buffer, ()>{
    let file = OpenOptions::new().read(true).open(path);
    if file.is_err() { return Err(()); }
    let mut opened = file.unwrap();
    let mut vec: Buffer = Vec::new();
    if opened.read_to_end(&mut vec).is_err() { return Err(()); }
    return Ok(vec);
}

#[derive(PartialEq)]
pub enum BufferFileType {
    Deadlines,
    Cards,
}

pub struct BufferFile{
    pub path: std::path::PathBuf,
    pub buffer: Option<Buffer>,
    bftype: BufferFileType,
    dirty: bool,
}

impl BufferFile{
    pub fn new(path: std::path::PathBuf, bftype: BufferFileType) -> BufferFile{
        BufferFile{
            path: path,
            buffer: Option::None,
            bftype: bftype,
            dirty: false,
        }
    }
    
    pub fn write(&mut self) -> bool{
        match self.buffer.as_mut(){
            Option::None =>{return false;}
            Option::Some(x) =>{
                if self.dirty{
                    self.dirty = !buffer_write_file(self.path.as_path(), &x);
                    return !self.dirty;
                }
                return true;
            }
        }
    }

    pub fn read(&mut self, force: bool) -> bool{
        fn _read(bf: &mut BufferFile) -> bool{
            let res = buffer_read_file(bf.path.as_path());
            match res{
                Err(_) => {return false;}
                Ok(x) => {
                    bf.buffer = Option::Some(x);
                    bf.dirty = false;
                    return true;
                }
            }
        }
        match self.buffer.as_mut(){
            Option::None =>{
                return _read(self);
            }
            Option::Some(_) =>{
                if !force {return false;}
                return _read(self);
            }
        }
    }

    pub fn add_deadline(&mut self, deadline: data::Deadline) -> bool{
        if self.bftype != BufferFileType::Deadlines {return false;}
        if self.buffer.is_none() {
            let ok = self.read(false);
            if !ok {return false;}
        }
        deadline.into_buffer(self.buffer.as_mut().unwrap());
        self.dirty = true;
        return true;
    }
}
