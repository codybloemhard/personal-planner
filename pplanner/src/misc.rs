pub trait DefaultValue{
    fn default_val() -> Self;
}

impl DefaultValue for u16{
    fn default_val() -> Self{
        0
    }
}

impl DefaultValue for bool{
    fn default_val() -> Self{
        false
    }
}

pub trait UnwrapDefault<T>{
    fn unwrap_default(res: Option<T>) -> T;
    fn replace_if_not_default(&mut self, new: T);
}

impl<T: DefaultValue + PartialEq> UnwrapDefault<T> for T{
    fn unwrap_default(res: Option<T>) -> T{
        if let Some(resv) = res{
            return resv;
        }
        T::default_val()
    }

    fn replace_if_not_default(&mut self, new: T){
        if new == Self::default_val() {return;}
        std::mem::replace(self, new);
    }
}

pub fn is_sorted<T: PartialOrd>(vec: &[T]) -> bool{
    let len = vec.len();
    if len <= 1 {return true;}
    if len == 2 {
        return vec[0] <= vec[1];
    }
    let mut last = &vec[0];
    for item in vec.iter().take(len).skip(1){
        if last > item {
            return false;
        }
        last = &item;
    }
    true
}
