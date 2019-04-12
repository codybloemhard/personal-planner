use std::io::prelude::*;
use std::fs::OpenOptions;

use super::data;

pub const DATA_DIR: &'static str = "~/.config/pplanner";
pub const DEADLINE_DIR: &'static str = "~/.config/pplanner/deadlines";

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

pub fn buffer_write_file(path: &str, vec: &Buffer) -> bool{
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(path);
    if file.is_err() { return false; }
    let mut opened = file.unwrap();
    if opened.write_all(&vec).is_err() { return false; }
    return true;
}

pub fn buffer_read_file(path: &str) -> Result<Buffer, ()>{
    let file = OpenOptions::new().read(true).open(&path);
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
    pub path: String,
    pub buffer: Option<Buffer>,
    bftype: BufferFileType,
    dirty: bool,
}

impl BufferFile{
    pub fn new(path: &str, bftype: BufferFileType) -> BufferFile{
        BufferFile{
            path: String::from(path),
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
                    self.dirty = !buffer_write_file(self.path.as_ref(), &x);
                    return self.dirty;
                }
                return true;
            }
        }
    }

    pub fn read(&mut self, force: bool) -> bool{
        fn _read(bf: &mut BufferFile) -> bool{
            let res = buffer_read_file(bf.path.as_ref());
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
        if self.buffer.is_none() {return false;}
        deadline.into_buffer(self.buffer.as_mut().unwrap());
        self.dirty = true;
        return true;
    }
}
