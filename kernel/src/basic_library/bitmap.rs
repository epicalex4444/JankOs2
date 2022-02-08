pub struct Bitmap{
    pub length: u64,
    pub bitmap_ptr: *mut u8,
}

impl Bitmap{
    pub fn new(start_addr: *mut u8, size: u64) -> Bitmap{
        unsafe{
            //let mut bitmap: *mut u8 = start_addr;
            for i in 0..size {
                *start_addr.offset(i as isize) = 0x00;
            }
            Bitmap{ length:size, bitmap_ptr: start_addr}
        }        
    }

    unsafe fn byte_from_index(&self, index: u64) -> *mut u8{
        self.bitmap_ptr.offset((index / 8) as isize)
    }

    // Gets the state of the bit at index
    pub fn get_bit(&self, index: u64) -> bool {
        if index < self.length * 8 {
            unsafe{
                let byte = self.byte_from_index(index);
                return *byte & 1 << index % 8 > 0; 
            }
            //return (self.bits & 1 << index) > 0;
        } else {
            core::panic("Index out of bounds of bitmap");
        }
    }

    // Attempts to set the bit at index, returns false if the index was out of bounds of the bitmap
    pub fn set_bit(&mut self, index: u64) -> bool {
        if index < self.length * 8 {
            unsafe{
                *(self.byte_from_index(index)) = *(self.byte_from_index(index)) | 1 << index%8;
            }            
            //self.bits = self.bits | 1 << index;
            return true;
        } else {
            return false;
        }
    }

    // Attempts to set the bit at index, returns false if the index was out of bounds of the bitmap
    pub fn clear_bit(&mut self, index: u64) -> bool {
        if index < self.length * 8 {
            unsafe{
                *(self.byte_from_index(index)) = *(self.byte_from_index(index)) & !(1 << index%8);
            }            
            //self.bits = self.bits | 1 << index;
            return true;
        } else {
            return false;
        }
    }
}
