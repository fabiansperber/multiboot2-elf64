//#[derive(Debug)]  // cannot use because of big arrays
#[repr(packed)]
pub struct VBEInfoTag {
    typ: u32,
    size: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    pub vbe_control_info: [u8; 512],
    pub vbe_mode_info: [u8; 256],
}

use core::fmt;
impl fmt::Debug for VBEInfoTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VBEInfoTag {{ typ: {}, size: {}, vbe_mode: {}, vbe_interface_seg: {}, vbe_interface_off: {}, vbe_interface_len: {} }}"
            , self.typ, self.size, self.vbe_mode, self.vbe_interface_seg, self.vbe_interface_off, self.vbe_interface_len)
    }
}
