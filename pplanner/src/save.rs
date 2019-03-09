use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

pub trait Binairizable{
    fn biniarize(&self) -> Vec<u8>;
}

pub fn test(){
    let mut file = File::create("test.save");
    if file.is_err() { return; }
}

pub fn buffer_append_u32(vec: &mut Vec<u8>, val: u32){
    vec.push(((val >> 24) & 0xff) as u8);
    vec.push(((val >> 16) & 0xff) as u8);
    vec.push(((val >> 8) & 0xff) as u8);
    vec.push((val & 0xff) as u8);
}

pub fn buffer_read_u32(vec: &Vec<u8>, iterator: &mut u32) -> (bool, u32){
    if (vec.len() as i32) - (*iterator as i32) < 4 { return (false,0); }
    let mut val: u32 = 0;
    val += (vec[(*iterator + 0) as usize] as u32) << 24;
    val += (vec[(*iterator + 1) as usize] as u32) << 16;
    val += (vec[(*iterator + 2) as usize] as u32) << 8;
    val += vec[(*iterator + 3) as usize] as u32;
    *iterator += 4;
    return (true,val);
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
    opened.read_to_end(&mut vec);
    return Ok(vec);
}