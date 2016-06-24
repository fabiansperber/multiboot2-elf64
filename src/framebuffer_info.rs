#[derive(Debug)]
#[repr(packed)]
pub struct FramebufferInfoTag {
    typ: u32,
    size: u32,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    framebuffer_type: u8,
    _reserved: u8,
    color_info: u8,     // Different data depending on framebuffer_type
}

#[derive(Debug)]
#[repr(packed)]
pub struct RgbColor(u8, u8, u8);

pub enum FrameBufferType {
    IndexedColor(RgbColorIter), // TODO: Better to return something indexable like a vector/slice
    DirectRGBColor(&'static RGBFieldInfo),
    /// EGA Text with 16-bit characters
    Text,
    /// This is not defined in the Multiboot spec, but is used as an error return value that should not happen
    Undefined,
}

impl FramebufferInfoTag {
    pub fn get_type(&'static self) -> FrameBufferType {
        match self.framebuffer_type {
            0 => unsafe {
                FrameBufferType::IndexedColor(RgbColorIter{
                    current_color: &*(((&self.color_info as *const _ as usize) + 4) as *const RgbColor),
                    remaining_colors: *((&self.color_info as *const _ as usize) as *const u32),
                })},
            1 => unsafe { FrameBufferType::DirectRGBColor(&*((&self.color_info as *const _ as usize) as *const RGBFieldInfo)) },
            2 => FrameBufferType::Text,
            _ => FrameBufferType::Undefined
        }
    }
}

pub struct RgbColorIter {
    current_color: &'static RgbColor,
    remaining_colors: u32,
}

impl Iterator for RgbColorIter {
    type Item = &'static RgbColor;
    fn next(&mut self) -> Option<&'static RgbColor> {
        if self.remaining_colors == 0 {
            None
        } else {
            let color = self.current_color;
            let next_color_addr = (self.current_color as *const _ as usize) + 3;
            self.current_color = unsafe{ &*(next_color_addr as *const RgbColor) };
            self.remaining_colors -= 1;
            Some(color)
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct RGBFieldInfo {
    red_field_position: u8,
    red_mask_size: u8,
    green_field_position: u8,
    green_mask_size: u8,
    blue_field_position: u8,
    blue_mask_size: u8,
}
