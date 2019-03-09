use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

pub trait Binairizable{
    fn biniarize(&self) -> Vec<u8>;
}

pub fn buffer_append_u32(vec: &mut Vec<u8>, val: u32){
    vec.push(((val >> 24) & 0xff) as u8);
    vec.push(((val >> 16) & 0xff) as u8);
    vec.push(((val >> 8) & 0xff) as u8);
    vec.push((val & 0xff) as u8);
}

pub fn buffer_read_u32(vec: &Vec<u8>, iter: &mut u32) -> (bool, u32){
    if (vec.len() as i32) - (*iter as i32) < 4 { return (false,0); }
    let mut val: u32 = 0;
    val += (vec[(*iter + 0) as usize] as u32) << 24;
    val += (vec[(*iter + 1) as usize] as u32) << 16;
    val += (vec[(*iter + 2) as usize] as u32) << 8;
    val += vec[(*iter + 3) as usize] as u32;
    *iter += 4;
    return (true,val);
}

pub fn buffer_append_string(vec: &mut Vec<u8>, string: &Vec<u8>){
    let len = string.len() as u32;
    buffer_append_u32(vec, len);
    for byte in string{
        vec.push(*byte);
    }
}

pub fn buffer_read_string(vec: &Vec<u8>, iter: &mut u32) -> (bool, Vec<u8>){
    let mut string: Vec<u8> = Vec::new();
    let res_len = buffer_read_u32(vec, iter);
    if !res_len.0 { return (false,string); }
    let len = res_len.1;
    if (vec.len() as i32) - (*iter as i32) < (len as i32) { return (false,string); }
    for i in *iter..(*iter+len){
        string.push(vec[i as usize]);
    }
    return (true,string);
}

pub fn buffer_write_file(path: &str, vec: &Vec<u8>) -> bool{
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(path);
    if file.is_err() { return false; }
    let mut opened = file.unwrap();
    if opened.write_all(&vec).is_err() { return false; }
    return true;
}

pub fn buffer_read_file(path: &str) -> Result<Vec<u8>,u8>{
    let file = OpenOptions::new().read(true).open(&path);
    if file.is_err() { return Err(0); }
    let mut opened = file.unwrap();
    let mut vec: Vec<u8> = Vec::new();
    if opened.read_to_end(&mut vec).is_err() { return Err(0); }
    return Ok(vec);
}