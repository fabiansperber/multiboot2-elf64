#![no_std]

pub use boot_loader_name::BootLoaderNameTag;
pub use elf_sections::{ElfSectionsTag, ElfSection, ElfSectionIter, ElfSectionType, ElfSectionFlags};
pub use elf_sections::{ELF_SECTION_WRITABLE, ELF_SECTION_ALLOCATED, ELF_SECTION_EXECUTABLE};
pub use memory_map::{MemoryMapTag, MemoryArea, MemoryAreaIter};
pub use modules::ModulesTag;
pub use framebuffer_info::{FramebufferInfoTag, FrameBufferType, RgbColor, RgbColorIter, RGBFieldInfo};
pub use vbe_info::VBEInfoTag;

#[macro_use]
extern crate bitflags;

mod boot_loader_name;
mod elf_sections;
mod memory_map;
mod modules;
mod framebuffer_info;
mod vbe_info;

pub unsafe fn load(address: usize) -> &'static BootInformation {
    let multiboot = &*(address as *const BootInformation);
    assert!(multiboot.has_valid_end_tag());
    multiboot
}

#[repr(C)]
pub struct BootInformation {
    pub total_size: u32,
    _reserved: u32,
    first_tag: Tag,
}

impl BootInformation {
    pub fn start_address(&self) -> usize {
        self as *const _ as usize
    }

    pub fn end_address(&self) -> usize {
        self.start_address() + self.total_size as usize
    }

    pub fn elf_sections_tag(&self) -> Option<&'static ElfSectionsTag> {
        self.get_tag(9).map(|tag| unsafe{&*(tag as *const Tag as *const ElfSectionsTag)})
    }

    pub fn memory_map_tag(&self) -> Option<&'static MemoryMapTag> {
        self.get_tag(6).map(|tag| unsafe{&*(tag as *const Tag as *const MemoryMapTag)})
    }

    pub fn boot_loader_name_tag(&self) -> Option<&'static BootLoaderNameTag> {
        self.get_tag(2).map(|tag| unsafe{&*(tag as *const Tag as *const BootLoaderNameTag)})
    }

    pub fn modules_tag(&self) -> Option<&'static ModulesTag> {
        self.get_tag(3).map(|tag| unsafe{&*(tag as *const Tag as *const ModulesTag)})
    }

    pub fn framebuffer_info_tag(&self) -> Option<&'static FramebufferInfoTag> {
        self.get_tag(8).map(|tag| unsafe{&*(tag as *const Tag as *const FramebufferInfoTag)})
    }

    pub fn vbe_info_tag(&self) -> Option<&'static VBEInfoTag> {
        self.get_tag(7).map(|tag| unsafe{&*(tag as *const Tag as *const VBEInfoTag)})
    }

    fn has_valid_end_tag(&self) -> bool {
        const END_TAG: Tag = Tag{typ:0, size:8};

        let self_ptr = self as *const _;
        let end_tag_addr = self_ptr as usize + (self.total_size - END_TAG.size) as usize;
        let end_tag = unsafe{&*(end_tag_addr as *const Tag)};

        end_tag.typ == END_TAG.typ && end_tag.size == END_TAG.size
    }

    fn get_tag(&self, typ: u32) -> Option<&'static Tag> {
        self.tags().find(|tag| tag.typ == typ)
    }

    fn tags(&self) -> TagIter {
        TagIter{current: &self.first_tag as *const _}
    }
}

#[repr(C)]
struct Tag {
    typ: u32,
    size: u32,
    // tag specific fields
}

struct TagIter {
    current: *const Tag,
}

impl Iterator for TagIter {
    type Item = &'static Tag;

    fn next(&mut self) -> Option<&'static Tag> {
        match unsafe{&*self.current} {
            &Tag{typ:0, size:8} => None, // end tag
            tag => {
                // go to next tag
                let mut tag_addr = self.current as usize;
                tag_addr += tag.size as usize;
                tag_addr = ((tag_addr-1) & !0x7) + 0x8; //align at 8 byte
                self.current = tag_addr as *const _;

                Some(tag)
            },
        }
    }
}
