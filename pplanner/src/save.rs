use std::io;
use std::fs::File;

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