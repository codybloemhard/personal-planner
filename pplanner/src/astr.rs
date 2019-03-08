pub type Astr = Vec<u8>;
pub type AstrVec = Vec<Vec<u8>>;

pub fn new() -> Astr{
    return Vec::new();
}

pub fn from_string(s: String) -> Astr{
    let mut buffer: Vec<u8> = Vec::new();
    for ch in s.chars() {
        if !ch.is_ascii() { continue; }
        let val: u8 = ch as u8;
        buffer.push(val);
    }
    return buffer;
}

pub fn from_str(s: &str) -> Astr{
    let mut buffer: Vec<u8> = Vec::new();
    for ch in s.chars(){
        if !ch.is_ascii() { continue; }
        let val: u8 = ch as u8;
        buffer.push(val);
    }
    return buffer;
}

pub fn clear(astr: &mut Astr){
    astr.clear();
}

pub fn to_string(astr: &Astr) -> String{
    let mut s: String = String::new();
    for ch in astr{
        s.push(*ch as char);
    }
    return s;
}

pub fn split(astr: &Astr, splitchars: &Astr) -> AstrVec{
    fn splitnow(splits: &mut AstrVec, current: &mut Astr, counter: &mut u32){
        splits.push(current.clone());
        current.clear();
        *counter = 0;
    }
    let mut splits: AstrVec = Vec::new();
    let mut current: Astr = Vec::new();
    let mut counter: u32 = 0;
    for ch in astr{
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