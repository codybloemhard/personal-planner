use std::io::prelude::*;
use std::fs::OpenOptions;

use super::conz;

pub const DATA_DIR: &'static str = ".config/pplanner";
pub const POINT_DIR: &'static str = "points";

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
        printer.println_type("Error: could not get home directory.", conz::MsgType::Error);
        return false;
    }
    let path = home.unwrap();
    let path = path.as_path();
    let metatdata = std::fs::metadata(path);
    let pathstr = path.to_str();
    if pathstr.is_none() {
        printer.println_type("Error: could not get string from path.", conz::MsgType::Error);
        return false;
    }
    let pathstr = pathstr.unwrap();
    if !metatdata.is_ok() {
        let res = std::fs::create_dir_all(path);
        if !res.is_ok() {
            printer.println_error("", "Error: Could not create path: ", pathstr);
            return false;
        }else{
            printer.print_type("First time use: created path: ", conz::MsgType::Highlight);
            printer.println_type(pathstr, conz::MsgType::Value);
        }
    }
    let dummy: Vec<u8> = Vec::new();
    {
        let pointpath = get_data_dir_path(POINT_DIR).unwrap();
        let pointpath = pointpath.as_path();
        let metatdata = std::fs::metadata(pointpath);
        if metatdata.is_err() {
            let ok = buffer_write_file(pointpath, &dummy);
            let pathstr = pointpath.to_str();
            if pathstr.is_none() {
                printer.print_type("Error: could not get string from path.", conz::MsgType::Error);
                return false;
            }
            let pathstr = pathstr.unwrap();
            if ok{
                printer.print_type("First time use: created path: ", conz::MsgType::Highlight);
                printer.println_type(pathstr, conz::MsgType::Value);
            }
            else{
                printer.println_error("", "Error: Could not create file: ", pathstr);
            }
        }
    }
    return true;
}

pub type Buffer = Vec<u8>;

pub trait Bufferable where Self: std::marker::Sized{
    fn into_buffer(&self, vec: &mut Buffer);
    fn from_buffer(vec: &Buffer, iter: &mut u32) -> Result<Self, ()>;
}

impl Bufferable for u32{
    fn into_buffer(&self, vec: &mut Buffer){
        vec.push(((*self >> 24) & 0xff) as u8);
        vec.push(((*self >> 16) & 0xff) as u8);
        vec.push(((*self >> 8) & 0xff) as u8);
        vec.push((*self & 0xff) as u8);
    }

    fn from_buffer(vec: &Buffer, iter: &mut u32) -> Result<Self, ()>{
        if (vec.len() as i32) - (*iter as i32) < 4 {return Err(());}
        let mut val: u32 = 0;
        val += (vec[(*iter + 0) as usize] as u32) << 24;
        val += (vec[(*iter + 1) as usize] as u32) << 16;
        val += (vec[(*iter + 2) as usize] as u32) << 8;
        val += vec[(*iter + 3) as usize] as u32;
        *iter += 4;
        return Ok(val);
    }
}

impl Bufferable for u8{
    fn into_buffer(&self, vec: &mut Buffer){
        vec.push(*self);
    }

    fn from_buffer(vec: &Buffer, iter: &mut u32) -> Result<Self, ()>{
        if (vec.len() as i32) - (*iter as i32) < 1 {return Err(());}
        let val = vec[*iter as usize];
        *iter += 1;
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
    if opened.write_all(&vec).is_err() {return false;}
    return true;
}

pub fn buffer_read_file(path: &std::path::Path) -> Result<Buffer, ()>{
    let file = OpenOptions::new().read(true).open(path);
    if file.is_err() { return Err(()); }
    let mut opened = file.unwrap();
    let mut vec: Buffer = Vec::new();
    if opened.read_to_end(&mut vec).is_err() {return Err(());}
    return Ok(vec);
}

pub struct BufferFile<T: Bufferable>{
    path: std::path::PathBuf,
    content: Vec<T>,
    dirty: bool,
    loaded: bool,
}

impl<T: Bufferable> BufferFile<T>{
    pub fn new(path: std::path::PathBuf) -> BufferFile<T>{
        BufferFile{
            path: path,
            content: Vec::new(),
            dirty: false,
            loaded: false,
        }
    }
    
    fn content_to_buffer(vec: &Vec<T>) -> Buffer{
        let mut buf = Vec::new();
        for x in vec{
            x.into_buffer(&mut buf);
        }
        return buf;
    }

    pub fn write(&mut self) -> bool{
        if !self.dirty{return true;}
        if !self.loaded{
            conz::printer().println_type("Error: Nothing to write, content was never initialized.", conz::MsgType::Error);
            return false;
        }
        self.dirty = !buffer_write_file(self.path.as_path(), &BufferFile::content_to_buffer(&self.content));
        if !self.dirty {return true;}
        let pathstr = self.path.to_str();
        if pathstr.is_none(){
            conz::printer().println_type("Error: Cannot get string from path.", conz::MsgType::Error);
        }else{
            conz::printer().println_error("", "Error: Cannot write items to file: ", pathstr.unwrap());
        }
        return false;
    }

    fn buffer_to_content(&mut self, vec: &Buffer){
        let mut iter: u32 = 0;
        self.content.clear();
        loop{
            let res = T::from_buffer(vec, &mut iter);
            if res.is_err() {break;}
            self.content.push(res.unwrap());
        }
        self.loaded = true;
    }

    pub fn read(&mut self, force: bool) -> bool{
        fn _read<T: Bufferable>(bf: &mut BufferFile<T>) -> bool{
            let res = buffer_read_file(bf.path.as_path());
            match res{
                Err(_) => {
                    let pathstr = bf.path.to_str();
                    if pathstr.is_none(){
                        conz::printer().println_type("Error: Cannot get string from path.", conz::MsgType::Error);
                    }else{
                        conz::printer().println_error("", "Error: Cannot read file: ", pathstr.unwrap());
                    }
                    return false;
                }
                Ok(x) => {
                    bf.buffer_to_content(&x);
                    bf.dirty = false;
                    return true;
                }
            }
        }
        if !self.loaded || force {
            return _read(self);
        }
        return true;
    }

    pub fn add_item(&mut self, item: T) -> bool{
        if !self.loaded{
            if !self.read(false) {
                conz::printer().println_type("Error: Cannot add item.", conz::MsgType::Error);
                return false;
            }
        }
        self.content.push(item);
        self.dirty = true;
        return true;
    }

    pub fn get_items(&self) -> &Vec<T>{
        return &self.content;
    }

    pub fn is_clean(&self) -> bool{
        return !self.dirty;
    }
}
