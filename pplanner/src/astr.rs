use super::save;
use super::misc::{DefaultValue};
use std::fmt;

pub type Astr = Vec<u8>;
pub type AstrVec = Vec<Vec<u8>>;

pub fn new() -> Astr{
    Vec::new()
}

pub fn from_str(s: &str) -> Astr{
    let mut buffer = new();
    for ch in s.chars(){
        if !ch.is_ascii() { continue; }
        let val: u8 = ch as u8;
        buffer.push(val);
    }
    buffer
}

pub trait ToAstr{
    fn to_astr(&self) -> Astr;
}

impl ToAstr for String{
    fn to_astr(&self) -> Astr{
        from_str(self)
    }
}

impl ToAstr for &'static str{
    fn to_astr(&self) -> Astr{
        from_str(self)
    }
}

pub struct DisplayableAstr{
    astr: Astr,
}

impl fmt::Display for DisplayableAstr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}", self.astr.to_string())
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
    fn disp(&self) -> DisplayableAstr;
    fn sameness(&self, other: &Astr) -> f32;
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
        s
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
        splits
    }

    fn copy_from_ref(&self) -> Astr{
        let mut newstr = new();
        for ch in self{
            newstr.push(*ch);
        }
        newstr
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
            newstr.push(b'.');
        }
        newstr
    }

    fn cut(&self, max: u16) -> Astr{
        let mut newstr = new();
        for i in 0..(std::cmp::min(max, std::cmp::max(max, self.len() as u16))){
            newstr.push(self[i as usize] as u8);
        }
        newstr
    }

    fn pad_after(&self, max: u16) -> Astr{
        if self.len() == max as usize {
            self.copy_from_ref()
        }else if self.len() < max as usize {
            let mut newstr = self.copy_from_ref();
            for _ in 0..(max-self.len() as u16){
                newstr.push(b' ');
            }
            newstr
        }else{
            self.confine(max)
        }
    }

    fn repeat(&self, times: u16) -> Astr{
        let mut newstr = new();
        for _ in 0..times{
            for ch in self{
                newstr.push(*ch);
            }
        }
        newstr
    }

    fn concat(&self, other: Astr) -> Astr{
        let mut newstr = self.copy_from_ref();
        for ch in other{
            newstr.push(ch);
        }
        newstr
    }

    fn to_lower(&self) -> Astr{
        let mut newstr = new();
        for ch in self{
            if char_is_letter_upper(*ch){
                newstr.push(ch - 26);
            }
            newstr.push(*ch);
        }
        newstr
    }

    fn disp(&self) -> DisplayableAstr{
        DisplayableAstr{
            astr: self.clone(),
        }
    }

    fn sameness(&self, other: &Astr) -> f32{
        if self == other { return 1.0; }
        let mut sum = 0.0;
        for sel in self{
            for oth in other{
                if sel == oth{
                    sum += 1.0;
                    break;
                }
            }
        }
        sum / std::cmp::max(self.len(), other.len()) as f32
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
        res_len?;
        let len = res_len.unwrap();
        if (vec.len() as i32) - (*iter as i32) < (len as i32) {
            return Option::None;
        }
        let mut string = new();
        for i in *iter..(*iter+len){
            string.push(vec[i as usize]);
        }
        *iter += len;
        Option::Some(string)
    }
}

impl DefaultValue for Astr{
    fn default_val() -> Self{
        new()
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
    newstr
}

//pub const CHAR_START_NUM: u8 = 48;
pub const CHAR_START_UPPER: u8 = 65;

/*pub fn char_is_num(ch: u8) -> bool{
    return ch >= CHAR_START_NUM && ch <= 57;
}*/

pub fn char_is_letter_upper(ch: u8) -> bool{
    ch >= CHAR_START_UPPER && ch <= 90
}

pub fn to_u32_checked(string: &Astr) -> Option<u32>{
    term_basics_linux::string_to_value(&string.to_string())
}

pub fn astr_whitespace() -> Astr{
    from_str(" \n\t")
}
