#[derive(Debug)]
#[repr(packed)]
pub struct ModulesTag {
    typ: u32,
    size: u32,
    pub mod_start: u32,
    pub mod_end: u32,
    string: u8,
}

use core::slice;
use core::mem;
use core::str;

impl ModulesTag {
    unsafe fn cast<T>(&self, addr: u32) -> Option<&T> {
        self.paddr_to_slice(addr, mem::size_of::<T>()).map(|inner| {
            mem::transmute(inner.as_ptr())
        })
    }

    unsafe fn paddr_to_slice(&self, p: u32, len: usize) -> Option<&'static [u8]> {
        let ptr: *const u8 = mem::transmute(p as usize);
        Some(slice::from_raw_parts(ptr, len))
    }

    unsafe fn convert_c_string(&self, string: u32) -> Option<&'static str> {
        if string == 0 {
            return None;
        }
        let mut len = 0;
        let mut ptr = string;
        while let Some(byte) = self.cast::<u8>(ptr) {
            if *byte == 0 {
                break;
            }
            ptr += 1;
            len += 1;
        }
        self.paddr_to_slice(string, len).map(|slice| str::from_utf8_unchecked(slice))
    }

    pub fn get_string(&self) -> Option<&'static str> {
        unsafe {
            self.convert_c_string(&self.string as *const _ as u32)
        }
    }

    pub fn get_module(&self) -> Option<&'static [u8]> {
        unsafe {
            self.paddr_to_slice(self.mod_start as u32, ((self.mod_end-1) - self.mod_start) as usize)
        }
    }
}
