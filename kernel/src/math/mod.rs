mod rounding;

pub trait RoundMath<T>{
    fn floor(&self, round:T) -> T;
    fn ceil(&self, round:T) -> T;
    fn round(&self, round:T) -> T;
}

pub fn maximum(a: u32, b: u32) -> u32 {
    return if a > b { a } else { b };
}

pub fn minimum(a: u32, b: u32) -> u32 {
    return if a < b { a } else { b };
}
