use std::io::prelude::*;
use std::fs::OpenOptions;
use simpleio as sio;

use super::conz;
use super::misc;

pub const DATA_DIR: &str = "pplanner";
pub const POINT_DIR: &str = "points";
pub const POINT_ARCHIVE_DIR: &str = "points_archive";
pub const PLAN_DIR: &str = "plans";
pub const PLAN_ARCHIVE_DIR: &str = "plans_archive";
pub const SLICE_DIR: &str = "slices";
pub const SLICE_ARCHIVE_DIR: &str = "slices_archive";
pub const TODO_DIR: &str = "todos";
pub const TODO_ARCHIVE_DIR: &str = "todos_archive";

pub fn get_data_dir_path(relative: &str) -> Option<std::path::PathBuf>{
    let confd = sio::get_config();
    confd.as_ref()?;
    let mut confd = confd.unwrap();
    confd.push(DATA_DIR);
    confd.push(relative);
    Option::Some(confd)
}

fn setup_file(p: &str){
    let pointpath = get_data_dir_path(p).unwrap();
    let pointpath = pointpath.as_path();
    let pathstr = pointpath.to_str();
    if pathstr.is_none() {
        conz::print_type("Error: could not get string from path.", conz::MsgType::Error);
        return;
    }
    let pathstr = pathstr.unwrap();
    if sio::file_exists(pointpath) { return; }
    let ok = buffer_write_file(pointpath, &Vec::new());
    if ok {
        conz::print_type("First time use: created path: ", conz::MsgType::Highlight);
        conz::println_type(pathstr, conz::MsgType::Value);
    }
    else {
        conz::println_error("", "Error: Could not create file: ", &pathstr);
        conz::println_type(pathstr, conz::MsgType::Value);
    }
}

pub fn setup_config_dir() -> bool{
    let conf = sio::get_config();
    if conf.is_none() {
        conz::println_type("Error: could not get config directory.", conz::MsgType::Error);
        return false;
    }
    let mut path = conf.unwrap();
    path.push(DATA_DIR);
    let path = path.as_path();
    let pathstr = path.to_str();
    if pathstr.is_none() {
        conz::println_type("Error: could not get string from path.", conz::MsgType::Error);
        return false;
    }
    let pathstr = pathstr.unwrap();
    match sio::create_dir(path){
        sio::DirStatus::Created =>{
            conz::print_type("First time use: created path: ", conz::MsgType::Highlight);
            conz::println_type(pathstr, conz::MsgType::Value);
        },
        sio::DirStatus::Error =>{
            conz::println_error("", "Error: Could not create path: ", &pathstr);
            return false;
        },
        _ =>{
            println!("{}", path.display());
        },
    }
    setup_file(POINT_DIR);
    setup_file(POINT_ARCHIVE_DIR);
    setup_file(PLAN_DIR);
    setup_file(PLAN_ARCHIVE_DIR);
    setup_file(SLICE_DIR);
    setup_file(SLICE_ARCHIVE_DIR);
    setup_file(TODO_DIR);
    setup_file(TODO_ARCHIVE_DIR);
    true
}

pub type Buffer = Vec<u8>;

pub trait Bufferable where Self: std::marker::Sized{
    fn into_buffer(&self, vec: &mut Buffer);
    fn from_buffer(vec: &Buffer, iter: &mut u32) -> Option<Self>;
}

impl Bufferable for u32{
    fn into_buffer(&self, vec: &mut Buffer){
        vec.push(((*self >> 24) & 0xff) as u8);
        vec.push(((*self >> 16) & 0xff) as u8);
        vec.push(((*self >> 8) & 0xff) as u8);
        vec.push((*self & 0xff) as u8);
    }

    fn from_buffer(vec: &Buffer, iter: &mut u32) -> Option<Self>{
        if (vec.len() as i32) - (*iter as i32) < 4 {return Option::None;}
        let mut val: u32 = 0;
        val += u32::from(vec[(*iter + 0) as usize]) << 24;
        val += u32::from(vec[(*iter + 1) as usize]) << 16;
        val += u32::from(vec[(*iter + 2) as usize]) << 8;
        val += u32::from(vec[(*iter + 3) as usize]);
        *iter += 4;
        Option::Some(val)
    }
}

impl Bufferable for u16{
    fn into_buffer(&self, vec: &mut Buffer){
        vec.push(((*self >> 8) & 0xff) as u8);
        vec.push((*self & 0xff) as u8);
    }

    fn from_buffer(vec: &Buffer, iter: &mut u32) -> Option<Self>{
        if (vec.len() as i32) - (*iter as i32) < 2 {return Option::None;}
        let mut val: u16 = 0;
        val += u16::from(vec[(*iter + 0) as usize]) << 8;
        val += u16::from(vec[(*iter + 1) as usize]);
        *iter += 2;
        Option::Some(val)
    }
}

impl Bufferable for u8{
    fn into_buffer(&self, vec: &mut Buffer){
        vec.push(*self);
    }

    fn from_buffer(vec: &Buffer, iter: &mut u32) -> Option<Self>{
        if (vec.len() as i32) - (*iter as i32) < 1 {return Option::None;}
        let val = vec[*iter as usize];
        *iter += 1;
        Option::Some(val)
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
    true
}

pub fn buffer_write_file_append(path: &std::path::Path, vec: &Buffer) -> bool{
    let file = OpenOptions::new().write(true).create(true).append(true).open(path);
    if file.is_err() { return false; }
    let mut opened = file.unwrap();
    if opened.write_all(&vec).is_err() {return false;}
    true
}

pub fn buffer_read_file(path: &std::path::Path) -> Option<Buffer>{
    let file = OpenOptions::new().read(true).open(path);
    if file.is_err() {return Option::None;}
    let mut opened = file.unwrap();
    let mut vec: Buffer = Vec::new();
    if opened.read_to_end(&mut vec).is_err() {return Option::None;}
    Option::Some(vec)
}

pub struct BufferFile<T: Bufferable + std::cmp::Ord>{
    path: std::path::PathBuf,
    content: Vec<T>,
    dirty: bool,
    loaded: bool,
    sorted: bool,
}

impl<T: Bufferable + std::cmp::Ord + Clone> BufferFile<T>{
    pub fn new(path: std::path::PathBuf) -> BufferFile<T>{
        BufferFile{
            path,
            content: Vec::new(),
            dirty: false,
            loaded: false,
            sorted: false,
        }
    }

    fn content_to_buffer(vec: &[T]) -> Buffer{
        let mut buf = Vec::new();
        for x in vec{
            x.into_buffer(&mut buf);
        }
        buf
    }

    pub fn write(&mut self) -> bool{
        if !self.dirty{return true;}
        if !self.loaded{
            conz::println_type("Error: Nothing to write, content was never initialized.", conz::MsgType::Error);
            return false;
        }
        if !self.sorted {self.sort(false);}
        self.dirty = !buffer_write_file(self.path.as_path(), &BufferFile::content_to_buffer(&self.content));
        if !self.dirty {return true;}
        let pathstr = self.path.to_str();
        if let Some(pathstrv) = pathstr{
            conz::println_error("", "Error: Cannot write items to file: ", &pathstrv);
        }else{
            conz::println_type("Error: Cannot get string from path.", conz::MsgType::Error);
        }
        false
    }

    fn buffer_to_content(&mut self, vec: &Buffer){
        let mut iter: u32 = 0;
        self.content.clear();
        loop{
            let res = T::from_buffer(vec, &mut iter);
            if res.is_none() {break;}
            self.content.push(res.unwrap());
        }
        self.loaded = true;
    }

    pub fn read(&mut self, force: bool) -> bool{
        fn _read<T: Bufferable + std::cmp::Ord + Clone>(bf: &mut BufferFile<T>) -> bool{
            let res = buffer_read_file(bf.path.as_path());
            match res{
                Option::None => {
                    let pathstr = bf.path.to_str();
                    if let Some(pathstrv) = pathstr{
                        conz::println_error("", "Error: Cannot read file: ", &pathstrv);
                    }else{
                        conz::println_type("Error: Cannot get string from path.", conz::MsgType::Error);
                    }
                    false
                }
                Option::Some(x) => {
                    bf.buffer_to_content(&x);
                    bf.dirty = false;
                    true
                }
            }
        }
        if !self.loaded || force {
            if !_read(self) {return false;}
            let sorted = misc::is_sorted(&self.content);
            if !sorted {
                conz::println_type("Warning: data was not stored sorted!", conz::MsgType::Error);
                self.sort(false);
                self.dirty = true;
            }
        }
        true
    }

    pub fn add_item(&mut self, item: T) -> bool{
        if !self.loaded && !self.read(false) {
            conz::println_type("Error: Cannot add item.", conz::MsgType::Error);
            return false;
        }
        self.content.push(item);
        self.dirty = true;
        self.sorted = false;
        true
    }

    pub fn get_items(&mut self) -> &Vec<T>{
        if !self.loaded {self.read(false);}
        self.sort(true);
        &self.content
    }

    pub fn sort(&mut self, check: bool){
        /*
        Rust docs:
        The current algorithm is an adaptive, iterative merge sort inspired by timsort.
        It is designed to be very fast in cases where the slice is nearly sorted, or consists of two or more sorted sequences concatenated one after another.
        *//*
        Items get added incrementally, written sorted, when first read there sorted.
        Should be ok-ish for our usecase.
        */
        if check && misc::is_sorted(&self.content){
            self.sorted = true;
            return;
        }
        self.content.sort();
        self.sorted = true;
    }

    pub fn is_clean(&self) -> bool{
        !self.dirty
    }

    pub fn remove_indices(&mut self, mut indices: Vec<usize>) -> bool{
        if !misc::is_sorted(&indices){
            conz::println_type("Warning: remove_indices, should be sorted, is not.", conz::MsgType::Error);
            indices.sort();
        }
        let mut index = 0;
        let mut vec = Vec::new();
        for i in 0..self.content.len(){
            if index < indices.len() && indices[index] == i {
                index += 1;
                continue;
            }
            vec.push(self.content[i].clone());
        }
        self.content = vec;
        self.dirty = true;
        self.write()
    }

    pub fn replace(&mut self, indices: Vec<usize>, replacements: Vec<T>) -> bool{
        if !misc::is_sorted(&indices){
            conz::println_type("Error: replace, indices should be sorted, is not.", conz::MsgType::Error);
            return false;
        }
        if indices.len() != replacements.len(){
            conz::println_type("Error: save::replace: indices.len() != replacements.len().", conz::MsgType::Error);
            return false;
        }
        let mut index = 0;
        for i in 0..self.content.len(){
            if index >= indices.len(){break;}
            if indices[index] == i {
                self.content[i] = replacements[index].clone();
                index += 1;
            }
        }
        self.sort(true);
        self.dirty = true;
        self.write()
    }
}

pub struct ArchiveFile<T: Bufferable>{
    path: std::path::PathBuf,
    content: Vec<T>,
    dirty: bool,
}

impl<T: Bufferable> ArchiveFile<T>{
    pub fn new(path: std::path::PathBuf) -> ArchiveFile<T>{
        ArchiveFile{
            path,
            content: Vec::new(),
            dirty: false,
        }
    }

    fn content_to_buffer(vec: &[T]) -> Buffer{
        let mut buf = Vec::new();
        for x in vec{
            x.into_buffer(&mut buf);
        }
        buf
    }

    pub fn write(&mut self) -> bool{
        if !self.dirty{return true;}
        self.dirty = !buffer_write_file_append(self.path.as_path(), &ArchiveFile::content_to_buffer(&self.content));
        if !self.dirty {
            self.content.clear();
            return true;
        }
        let pathstr = self.path.to_str();
        if let Some(pathstrv) = pathstr {
            conz::println_error("", "Error: Cannot write items to file: ", &pathstrv);
        }else{
            conz::println_type("Error: Cannot get string from path.", conz::MsgType::Error);
        }
        false
    }

    fn buffer_to_content(&mut self, vec: &Buffer) -> Vec<T>{
        let mut iter: u32 = 0;
        let mut ret = Vec::new();
        loop{
            let res = T::from_buffer(vec, &mut iter);
            if res.is_none() {break;}
            ret.push(res.unwrap());
        }
        ret
    }

    pub fn read(&mut self) -> Vec<T>{
        let res = buffer_read_file(self.path.as_path());
        match res{
            Option::None => {
                let pathstr = self.path.to_str();
                if let Some(pathstrv) = pathstr{
                    conz::println_error("", "Error: Cannot read file: ", &pathstrv);
                }else{
                    conz::println_type("Error: Cannot get string from path.", conz::MsgType::Error);
                }
                Vec::new()
            }
            Option::Some(x) => {
                self.buffer_to_content(&x)
            }
        }
    }

    pub fn add_item(&mut self, item: T){
        self.content.push(item);
        self.dirty = true;
    }

    pub fn is_clean(&self) -> bool{
        !self.dirty
    }
}
