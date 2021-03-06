use io::*;
use interrupt::*;
/* the keyboard gives us a a number that isn't ascii (there are non-ascii keys
 * on the keyboard). This array maps the keyboard's scancode to characters */
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

#[allow(unused_variables)]
pub fn keyboard(regs: Registers) {
    unsafe {
        let scancode: u8 = ::io::inportb(0x60);
        if (scancode & 0x80) == 0 {
            // echo the key
            print!("{}", KEYMAP[scancode as usize] as char);
        }
    }
}

pub fn init()
{
    ::interrupt::register_handler(1, keyboard);
}

