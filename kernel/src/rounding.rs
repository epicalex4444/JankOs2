pub trait RoundMath<T>{
    fn floor(&self, round:T) -> T;
    fn ceil(&self, round:T) -> T;
    fn round(&self, round:T) -> T;
}

impl RoundMath<u64> for u64 {
    fn floor(&self, round: u64) -> u64 {
        return (self / round) * round;
    }

    fn ceil(&self, round: u64) -> u64 {
        return (self / round + if self % round != 0 {1} else {0}) * round;
    }

    fn round(&self, round: u64) -> u64{
        if (self % round) > (round / 2) {
            return (self / round + 1) * round;
        } else {
            return self / round * round;
        }
    }
}

impl RoundMath<u32> for u32 {
    fn floor(&self, round: u32) -> u32 {
        return self / round * round;
    }

    fn ceil(&self, round: u32) -> u32 {
        return (self / round + if self % round != 0 {1} else {0}) * round;
    }

    fn round(&self, round: u32) -> u32 {
        if (self % round) > (round / 2) {
            return (self / round + 1) * round;
        } else {
            return self / round * round;
        } 
    }
}

impl RoundMath<i64> for i64{
    fn floor(&self, round: i64) -> i64 {
        return (self / round) * round;
    }

    fn ceil(&self, round: i64) -> i64{
        if self < &0 {
            return (self / round - if self % round != 0 {1} else {0}) * round;
        } else {
            return (self / round + if self % round != 0 {1} else {0}) * round;
        }
    }

    fn round(&self, round: i64) -> i64 {
        if self < &0 {
            if (self % round) <= (round / 2) {
                return ((self / round) - 1) * round;
            } else {
                return self / round * round;
            }            
        } else {
            if (self % round) >= (round / 2) {
                return ((self / round) + 1) * round;
            } else {
                return self / round * round;
            }  
        }
    }
}

impl RoundMath<i32> for i32 {
    fn floor(&self, round: i32) -> i32 {
        return (self / round) * round;
    }

    fn ceil(&self, round: i32) -> i32{
        if self < &0 {
            return (self / round - if self % round != 0 {1} else {0}) * round;
        } else {
            return (self / round + if self % round != 0 {1} else {0}) * round;
        }
        
    }

    fn round(&self, round: i32) -> i32 {
        if self < &0 {
            if (self % round) <= (round / 2) {
                return (self / round - 1) * round;
            } else {
                return self / round * round;
            }            
        } else {
            if (self % round) >= (round / 2) {
                return (self / round + 1) * round;
            } else {
                return self / round * round;
            }  
        }
    }
}