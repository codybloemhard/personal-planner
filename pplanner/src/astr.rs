type Astr = Vec<u8>;

pub fn new() -> Astr{
    return Vec::new();
}

pub fn from(s: String) -> Astr{
    let mut buffer: Vec<u8> = Vec::new();
    for ch in s.chars() {
        if !ch.is_ascii() { continue; }
        let val: u8 = ch as u8;
        buffer.push(val);
    }
    return buffer;
}

pub fn to_string(astr: Astr) -> String{
    let mut s: String = String::new();
    for ch in &astr{
        s.push(*ch as char);
    }
    return s;
}
