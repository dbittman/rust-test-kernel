use core::*;
use core::mem::*;
use vga::*;
use io::{outportb,inportb};
// This is needed to ensure that the structure actually corrosponds to
// what the CPU expects. Correctly ordered, and packed.
#[repr(C, packed)]
#[derive(Copy)]
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
pub static mut gdt: [gdt_desc; 3] = [
    gdt_desc {  base_low:0, base_mid:0, base_high:0,
        limit_low:0, access:0, gran:0 },
    gdt_desc {  base_low:0, base_mid:0, base_high:0,
        limit_low:0xFFFF, access:0x9A, gran:0xCF },
    gdt_desc {  base_low:0, base_mid:0, base_high:0,
        limit_low:0xFFFF, access:0x92, gran:0xCF },
];

// This is defined in boot.s
extern "C" {
    fn reload_segments();
    fn load_idt(x: u32, y: u32);
    fn int32_entry();
    fn int33_entry();
}

pub unsafe fn gdt_init()
{
    // Create a gdt pointer, and load it into the GDTR register
    let g: gdt_ptr = gdt_ptr {
        table: &gdt,
        length: 23
    };
    asm!("lgdt ($0)" :: "r"(&g as *const gdt_ptr));
    reload_segments();
}

pub static mut idt: [u64; 256] = [0; 256];

pub unsafe fn idt_write_entry(idx: u8, vector: unsafe extern fn())
{
    let addr: u32 = ::core::mem::transmute(vector);
    idt[idx as usize] = 
        (addr & 0xFFFF) as u64 | 
        (0x8 << 16) as u64 |
        /* bits 32..40 are always zero */
        ((0x8E as u64) << 40) |
        (((addr >> 16) & 0xFFFF) as u64) << 48;
}

pub unsafe fn idt_init()
{
    idt_write_entry(32, int32_entry);
    idt_write_entry(33, int33_entry);
    load_idt(::core::mem::transmute(&idt as *const [u64; 256]), 16383);
}

pub unsafe fn pic_init()
{
    outportb(0x20, 0x11);
    outportb(0xA0, 0x11);
    outportb(0x21, 0x20);
    outportb(0xA1, 0x28);
    outportb(0x21, 0x04);
    outportb(0xA1, 0x02);
    outportb(0x21, 0x01);
    outportb(0xA1, 0x01);
    outportb(0x21, 0x0);
    outportb(0xA1, 0x0);
    //outportb(0x21,0xfd);
    //outportb(0xa1,0xff);

    let divisor: u32 = 1193180 / 100;       /* Calculate our divisor */
    outportb(0x43, 0x36);             /* Set our command byte 0x36 */
    outportb(0x40, (divisor & 0xFF) as u8);   /* Set low byte of divisor */
    outportb(0x40, (divisor >> 8) as u8);     /* Set high byte of divisor */
}

