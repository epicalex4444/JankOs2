pub struct Bitmap {
    pub bits: u8,
}

impl Bitmap {
    // Initialise new bitmap, a byte where each bit is treated as a boolean
    pub fn new() -> Bitmap {
        Bitmap { bits: 0 }
    }

    // Gets teh state of the bit at index
    pub fn get_bit(&self, index: u8) -> bool {
        if index < 8 {
            return (self.bits & 1 << index) > 0;
        } else {
            return false;
        }
    }

    // Sets the state of the bit at index to state. Returns true if bit was succesfully set or false if something went wrong
    pub fn set_bit(&mut self, index: u8, state: bool) -> bool {
        if index < 8 {
            if state {
                self.bits = self.bits | 1 << index;
            } else {
                self.bits = self.bits & !(1 << index);
            }
            return true;
        } else {
            return false;
        }
    }
}
