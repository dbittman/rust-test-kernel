use core::*;
// This is needed to ensure that the structure actually corrosponds to
// what the CPU expects. Correctly ordered, and packed.
#[repr(C, packed)]
pub struct gdt_desc {
    limit_low: u16,
    base_low : u16,
    base_mid : u8,
    access   : u8,
    gran     : u8,
    base_high: u8
}

// This one for sure needs to be packed because it starts with a u16!!
#[repr(C, packed)]
struct gdt_ptr {
    length: u16,
    table: *const [gdt_desc],

}

// Basic ring-0 only GDT. Flat addressing mode, with a code and data segment.
pub static mut gdt: [gdt_desc; 3] = unsafe { [
        gdt_desc {  base_low:0, base_mid:0, base_high:0,
                    limit_low:0, access:0, gran:0 },
        gdt_desc {  base_low:0, base_mid:0, base_high:0,
                    limit_low:0xFFFF, access:0x9A, gran:0xCF },
        gdt_desc {  base_low:0, base_mid:0, base_high:0,
                    limit_low:0xFFFF, access:0x92, gran:0xCF },
    ]};

// This is defined in boot.s
extern "C" {
    fn reload_segments();
}

pub fn gdt_init()
{
    // Create a gdt pointer, and load it into the GDTR register
    unsafe {
        let g: gdt_ptr = gdt_ptr {
            table: &gdt,
            length: 23
        };
        asm!("lgdt ($0)" :: "r"(&g as *const gdt_ptr));
        reload_segments();
    }
}

pub fn idt_init()
{
    
}

pub fn pic_init()
{

}

pub fn sti()
{
    unsafe {
        asm!("sti");
    }
}

pub fn cli()
{
    unsafe {
        asm!("cli");
    }
}

