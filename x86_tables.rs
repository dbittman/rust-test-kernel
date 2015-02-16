use core::*;
use core::mem::*;
use vga::*;
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

// This one for sure needs to be packed because it starts with a u16!!
#[repr(C, packed)]
struct idt_ptr {
    length: u16,
    table: *const [u64],
}
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
    let p: idt_ptr = idt_ptr {
        table: &idt,
        length: 16383
    };
    asm!("lidt ($0)" :: "r"(&p as *const idt_ptr));
}

unsafe fn outportb(port: u16, val: u8)
{
    asm!("outb %al, %dx" : : "{dx}"(port), "{al}"(val));
}

unsafe fn inportb(port: u16) -> u8
{
    let ret: u8;
    asm!("inb %dx, %al" : "={ax}"(ret): "{dx}"(port));
    ret
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
    outportb(0x21,0xfd);
    outportb(0xa1,0xff);

    let divisor: u32 = 1193180 / 100;       /* Calculate our divisor */
    outportb(0x43, 0x36);             /* Set our command byte 0x36 */
    outportb(0x40, (divisor & 0xFF) as u8);   /* Set low byte of divisor */
    outportb(0x40, (divisor >> 8) as u8);     /* Set high byte of divisor */
}

#[derive(Copy)]
#[repr(C, packed)]
pub struct Registers {
    ds: u32,
    edi: u32, esi: u32, ebp: u32, esp: u32, ebx: u32, edx: u32, ecx: u32, eax: u32,
    int_num: u32, err_code: u32,
    eip: u32, cs: u32, eflags: u32, useresp: u32, ss: u32
}

static mut ints: u32 = 0;

static KEYMAP: [u8; 79] = [
    0, 27,
    '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8, '8' as u8, /* 9 */
  '9' as u8, '0' as u8, '-' as u8, '=' as u8, 8, /* Backspace */
  '\t' as u8,         /* Tab */
  'q' as u8, 'w' as u8, 'e' as u8, 'r' as u8,   /* 19 */
  't' as u8, 'y' as u8, 'u' as u8, 'i' as u8, 'o' as u8, 'p' as u8, '[' as u8, ']' as u8, '\n' as u8, /* Enter key */
    0,          /* 29   - Control */
  'a' as u8, 's' as u8, 'd' as u8, 'f' as u8, 'g' as u8, 'h' as u8, 'j' as u8, 'k' as u8, 'l' as u8, ';' as u8, /* 39 */
 '\'' as u8, '`' as u8,   0,        /* Left shift */
 '\\' as u8, 'z' as u8, 'x' as u8, 'c' as u8, 'v' as u8, 'b' as u8, 'n' as u8,            /* 49 */
  'm' as u8, ',' as u8, '.' as u8, '/' as u8,   0,              /* Right shift */
  '*' as u8,
    0,  /* Alt */
  ' ' as u8,  /* Space bar */
    0,  /* Caps lock */
    0,  /* 59 - F1 key ... > */
    0,   0,   0,   0,   0,   0,   0,   0,
    0,  /* < ... F10 */
    0,  /* 69 - Num lock*/
    0,  /* Scroll Lock */
    0,  /* Home key */
    0,  /* Up Arrow */
    0,  /* Page Up */
  '-' as u8,
    0,  /* Left Arrow */
    0,
    0,  /* Right Arrow */
  '+' as u8,
];  

#[no_mangle]
pub unsafe extern "C" fn interrupt_handler(regs: Registers)
{
    //print!("Interrupt: {}\n", ints);
    ints += 1;
    if regs.int_num == 33 {
        let scancode: u8 = inportb(0x60);
        if((scancode & 0x80) == 0) {
            print!("{}", KEYMAP[scancode as usize] as char);
        }
    }
    if regs.int_num >= 40 {
        outportb(0xA0, 0x20);
    }
    outportb(0x20, 0x20);
}

pub unsafe fn sti()
{
    asm!("sti");
}

pub unsafe fn cli()
{
    asm!("cli");
}

