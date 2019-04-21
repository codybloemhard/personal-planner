pub trait DefaultValue{
    fn default_val() -> Self;
}

pub trait UnwrapDefault<T>{
    fn unwrap_default(res: Option<T>) -> T;
    fn replace_if_not_default(&mut self, new: T);
}

impl<T: DefaultValue + PartialEq> UnwrapDefault<T> for T{
    fn unwrap_default(res: Option<T>) -> T{
        if res.is_some(){
            return res.unwrap();
        }
        return T::default_val();
    }

    fn replace_if_not_default(&mut self, new: T){
        if new == Self::default_val() {return;}
        std::mem::replace(self, new);
    }
}

pub fn is_sorted<T: PartialOrd>(vec: &Vec<T>) -> bool{
    let len = vec.len();
    if len <= 1 {return true;}
    if len == 2 {
        return vec[0] <= vec[1];
    }
    let mut last = &vec[0];
    for i in 1..len{
        if last > &vec[i] {
            return false;
        }
        last = &vec[i];
    }
    return true;
}
