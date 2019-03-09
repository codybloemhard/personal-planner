use std::io::prelude::*;
use std::fs::OpenOptions;

pub trait Bufferable{
    type Return;
    fn into_buffer(&self, vec: &mut Vec<u8>);
    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Result<Self::Return, ()>;
}

impl Bufferable for u32{
    type Return = u32;
    fn into_buffer(&self, vec: &mut Vec<u8>){
        vec.push(((*self >> 24) & 0xff) as u8);
        vec.push(((*self >> 16) & 0xff) as u8);
        vec.push(((*self >> 8) & 0xff) as u8);
        vec.push((*self & 0xff) as u8);
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Result<Self::Return, ()>{
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

pub fn buffer_append_buffer(vec: &mut Vec<u8>, string: &Vec<u8>){
    for byte in string{
        vec.push(*byte);
    }
}

pub fn buffer_append_string(vec: &mut Vec<u8>, string: &Vec<u8>){
    let len = string.len() as u32;
    u32::into_buffer(&len, vec);
    for byte in string{
        vec.push(*byte);
    }
}

pub fn buffer_read_string(vec: &Vec<u8>, iter: &mut u32) -> Result<Vec<u8>, ()>{
    let res_len = u32::from_buffer(vec, iter);
    if res_len.is_err() { return Err(()); }
    let len = res_len.unwrap();
    if (vec.len() as i32) - (*iter as i32) < (len as i32) { return Err(()); }
    let mut string: Vec<u8> = Vec::new();
    for i in *iter..(*iter+len){
        string.push(vec[i as usize]);
    }
    *iter += len;
    return Ok(string);
}

pub fn buffer_write_file(path: &str, vec: &Vec<u8>) -> bool{
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(path);
    if file.is_err() { return false; }
    let mut opened = file.unwrap();
    if opened.write_all(&vec).is_err() { return false; }
    return true;
}

pub fn buffer_read_file(path: &str) -> Result<Vec<u8>, ()>{
    let file = OpenOptions::new().read(true).open(&path);
    if file.is_err() { return Err(()); }
    let mut opened = file.unwrap();
    let mut vec: Vec<u8> = Vec::new();
    if opened.read_to_end(&mut vec).is_err() { return Err(()); }
    return Ok(vec);
}