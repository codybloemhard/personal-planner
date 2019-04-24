use super::save;
use super::misc::{DefaultValue};

pub type Astr = Vec<u8>;
pub type AstrVec = Vec<Vec<u8>>;

pub fn new() -> Astr{
    return Vec::new();
}

pub fn from_string(s: String) -> Astr{
    let mut buffer = new();
    for ch in s.chars() {
        if !ch.is_ascii() { continue; }
        let val: u8 = ch as u8;
        buffer.push(val);
    }
    return buffer;
}

pub fn from_str(s: &str) -> Astr{
    let mut buffer = new();
    for ch in s.chars(){
        if !ch.is_ascii() { continue; }
        let val: u8 = ch as u8;
        buffer.push(val);
    }
    return buffer;
}

pub trait ToAstr{
    fn to_astr(self) -> Astr;
}

impl ToAstr for String{
    fn to_astr(self) -> Astr{
        return from_string(self);
    }
}

impl ToAstr for &'static str{
    fn to_astr(self) -> Astr{
        return from_str(self);
    }
}

pub trait TOSTRING{
    fn tostring(&self) -> std::string::String;
}

impl TOSTRING for &str{
    fn tostring(&self) -> std::string::String{
        return std::string::String::from(*self);
    }
}

impl TOSTRING for std::string::String{
    fn tostring(&self) -> std::string::String{
        return self.clone();
    }   
}

impl TOSTRING for Astr{
    fn tostring(&self) -> std::string::String{
        return self.to_string();
    }   
}

pub trait AStr{
    fn clear(&mut self);
    fn to_string(&self) -> std::string::String;
    fn split_str(&self, splitchars: &Astr) -> AstrVec;
    fn copy_from_ref(&self) -> Astr;
    fn confine(&self, max: u16) -> Astr;
    fn pad_after(&self, max: u16) -> Astr;
    fn repeat(&self, times: u16) -> Astr;
    fn concat(&self, other: Astr) -> Astr;
    fn to_lower(&self) -> Astr;
    fn cut(&self, max: u16) -> Astr;
}

impl AStr for Astr{
    fn clear(&mut self){
        self.clear();
    }

    fn to_string(&self) -> std::string::String{
        let mut s: String = String::new();
        for ch in self{
            s.push(*ch as char);
        }
        return s;
    }

    fn split_str(&self, splitchars: &Astr) -> AstrVec{
        fn splitnow(splits: &mut AstrVec, current: &mut Astr, counter: &mut u32){
            splits.push(current.clone());
            current.clear();
            *counter = 0;
        }
        let mut splits: AstrVec = Vec::new();
        let mut current = new();
        let mut counter: u32 = 0;
        for ch in self{
            let mut hit: bool = false;
            for sp in splitchars{
                if *ch == *sp{
                    hit = true;
                    break;
                }
            }
            if hit{
                if counter == 0{
                    continue;
                }
                splitnow(&mut splits, &mut current, &mut counter);
                continue;
            }
            counter += 1;
            current.push(*ch);
        }
        if counter > 0{
            splitnow(&mut splits, &mut current, &mut counter);
        }
        return splits;
    }

    fn copy_from_ref(&self) -> Astr{
        let mut newstr = new();
        for ch in self{
            newstr.push(*ch);
        }
        return newstr;
    }

    fn confine(&self, max: u16) -> Astr{
        if self.len() <= max as usize {
            return self.copy_from_ref();
        }
        let mut newstr = new();
        for i in 0..(max-3){
            newstr.push(self[i as usize] as u8);
        }
        for _ in 0..3 {
            newstr.push('.' as u8);
        }
        return newstr;
    }

    fn cut(&self, max: u16) -> Astr{
        let mut newstr = new();
        for i in 0..(std::cmp::min(max, std::cmp::max(max, self.len() as u16))){
            newstr.push(self[i as usize] as u8);
        }
        return newstr;
    }

    fn pad_after(&self, max: u16) -> Astr{
        if self.len() == max as usize {
            return self.copy_from_ref();
        }else if self.len() < max as usize {
            let mut newstr = self.copy_from_ref();
            for _ in 0..(max-self.len() as u16){
                newstr.push(' ' as u8);
            }
            return newstr;
        }else{
            return self.confine(max);
        }
    }

    fn repeat(&self, times: u16) -> Astr{
        let mut newstr = new();
        for _ in 0..times{
            for ch in self{
                newstr.push(*ch);
            }
        }
        return newstr;
    }

    fn concat(&self, other: Astr) -> Astr{
        let mut newstr = self.copy_from_ref();
        for ch in other{
            newstr.push(ch);
        }
        return newstr;
    }

    fn to_lower(&self) -> Astr{
        let mut newstr = new();
        for ch in self{
            if char_is_letter_upper(*ch){
                newstr.push(ch - 26);
            }
            newstr.push(*ch);
        }
        return newstr;
    }
}

impl save::Bufferable for Astr{
    //type Return = Astr;
    fn into_buffer(&self, vec: &mut Vec<u8>){
        let len = self.len() as u32;
        u32::into_buffer(&len, vec);
        save::buffer_append_buffer(vec, self);
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Option<Self>{
        let res_len = u32::from_buffer(vec, iter);
        if res_len.is_none() {return Option::None;}
        let len = res_len.unwrap();
        if (vec.len() as i32) - (*iter as i32) < (len as i32) {
            return Option::None;
        }
        let mut string = new();
        for i in *iter..(*iter+len){
            string.push(vec[i as usize]);
        }
        *iter += len;
        return Option::Some(string);
    }
}

impl DefaultValue for Astr{
    fn default_val() -> Self{
        return new();
    }
}

pub fn unsplit(vec: &AstrVec, divider: u8) -> Astr{
    let mut newstr = new();
    let mut counter = 0;
    let max = vec.len() - 1;
    for v in vec{
        for ch in v{
            newstr.push(*ch);
        }
        if counter != max{
            newstr.push(divider);
        }
        counter+=1;
    }
    return newstr;
}

pub const CHAR_START_NUM: u8 = 48;
pub const CHAR_START_UPPER: u8 = 65;
pub const CHAR_START_LOWER: u8 = 96;

pub fn char_is_normal(ch: u8) -> bool{
    return (ch >= 32 && ch <= 126) || ch == 9 || ch == 10;
}

pub fn char_is_num(ch: u8) -> bool{
    return ch >= CHAR_START_NUM && ch <= 57;
}

pub fn char_is_letter_upper(ch: u8) -> bool{
    return ch >= CHAR_START_UPPER && ch <= 90;
}

pub fn char_is_letter_lower(ch: u8) -> bool{
    return ch >= CHAR_START_LOWER && ch <= 122;
}

pub fn char_is_letter(ch: u8) -> bool{
    return char_is_letter_lower(ch) || char_is_letter_upper(ch);
}

pub fn to_u32_unchecked(string: &Astr) -> u32{
    let mut u: u32 = 0;
    for ch in string{
        u = u * 10 + ((ch - 48) as u32);
    }
    return u;
}

pub fn astr_whitespace() -> Astr{
    return from_str(" \n\t");
}