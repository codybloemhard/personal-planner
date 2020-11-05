use super::save;
use std::fmt;

#[derive(PartialEq,Eq,PartialOrd,Ord,Hash,Clone)]
pub struct Astr(pub Vec<u8>);
pub type AstrVec = Vec<Astr>;

pub fn from_str(s: &str) -> Astr{
    let mut buffer = Vec::new();
    for ch in s.chars(){
        if !ch.is_ascii() { continue; }
        let val: u8 = ch as u8;
        buffer.push(val);
    }
    Astr(buffer)
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
    fn new() -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
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
    fn new() -> Self{
        Astr(Vec::new())
    }

    fn len(&self) -> usize{
        self.0.len()
    }

    fn is_empty(&self) -> bool{
        self.0.is_empty()
   }
    fn clear(&mut self){
        self.0.clear();
    }

    fn to_string(&self) -> std::string::String{
        let mut s: String = String::new();
        for ch in &self.0{
            s.push(*ch as char);
        }
        s
    }

    fn split_str(&self, splitchars: &Astr) -> AstrVec{
        fn splitnow(splits: &mut AstrVec, current: &mut Astr, counter: &mut u32){
            splits.push(current.clone());
            current.0.clear();
            *counter = 0;
        }
        let mut splits: AstrVec = Vec::new();
        let mut current = Self::new();
        let mut counter: u32 = 0;
        for ch in &self.0{
            let mut hit: bool = false;
            for sp in &splitchars.0{
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
            current.0.push(*ch);
        }
        if counter > 0{
            splitnow(&mut splits, &mut current, &mut counter);
        }
        splits
    }

    fn copy_from_ref(&self) -> Astr{
        let mut newstr = Vec::new();
        for ch in &self.0{
            newstr.push(*ch);
        }
        Astr(newstr)
    }

    fn confine(&self, max: u16) -> Astr{
        if self.len() <= max as usize {
            return self.copy_from_ref();
        }
        let mut newstr = Vec::new();
        for i in 0..(max-3){
            newstr.push(self.0[i as usize] as u8);
        }
        for _ in 0..3 {
            newstr.push(b'.');
        }
        Astr(newstr)
    }

    fn cut(&self, max: u16) -> Astr{
        let mut newstr = Vec::new();
        for i in 0..(std::cmp::min(max, std::cmp::max(max, self.len() as u16))){
            newstr.push(self.0[i as usize] as u8);
        }
        Astr(newstr)
    }

    fn pad_after(&self, max: u16) -> Astr{
        if self.len() == max as usize {
            self.copy_from_ref()
        }else if self.len() < max as usize {
            let mut newstr = self.copy_from_ref();
            for _ in 0..(max-self.len() as u16){
                newstr.0.push(b' ');
            }
            newstr
        }else{
            self.confine(max)
        }
    }

    fn repeat(&self, times: u16) -> Astr{
        let mut newstr = Vec::new();
        for _ in 0..times{
            for ch in &self.0{
                newstr.push(*ch);
            }
        }
        Astr(newstr)
    }

    fn concat(&self, other: Astr) -> Astr{
        let mut newstr = self.copy_from_ref();
        for ch in other.0{
            newstr.0.push(ch);
        }
        newstr
    }

    fn to_lower(&self) -> Astr{
        let mut newstr = Vec::new();
        for ch in &self.0{
            if char_is_letter_upper(*ch){
                newstr.push(ch - 26);
            }
            newstr.push(*ch);
        }
        Astr(newstr)
    }

    fn disp(&self) -> DisplayableAstr{
        DisplayableAstr{
            astr: self.clone(),
        }
    }

    fn sameness(&self, other: &Astr) -> f32{
        if self.0 == other.0 { return 1.0; }
        let mut sum = 0.0;
        for sel in &self.0{
            for oth in &other.0{
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
        save::buffer_append_buffer(vec, &self.0);
    }

    fn from_buffer(vec: &Vec<u8>, iter: &mut u32) -> Option<Self>{
        let res_len = u32::from_buffer(vec, iter);
        res_len?;
        let len = res_len.unwrap();
        if (vec.len() as i32) - (*iter as i32) < (len as i32) {
            return Option::None;
        }
        let mut string = Vec::new();
        for i in *iter..(*iter+len){
            string.push(vec[i as usize]);
        }
        *iter += len;
        Option::Some(Astr(string))
    }
}

impl Default for Astr{
    fn default() -> Self{
        Self::new()
    }
}

pub fn unsplit(vec: &AstrVec, divider: u8) -> Astr{
    let mut newstr = Vec::new();
    let mut counter = 0;
    let max = vec.len() - 1;
    for v in vec{
        for ch in &v.0{
            newstr.push(*ch);
        }
        if counter != max{
            newstr.push(divider);
        }
        counter+=1;
    }
    Astr(newstr)
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
