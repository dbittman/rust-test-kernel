use prelude::*;
use util::*;
use core::fmt;

const WHITE_ON_BLACK: u16 = (0 << 12) | (15 << 8);

pub struct Display {
    x: isize,
    y: isize,
    screen: *mut u16
}

pub static mut mainscreen: Display = Display { x:0, y:0, screen:0xb8000 as *mut u16 };

impl Display {
    pub fn new() -> Display {
        Display {x:0, y:0, screen:0xb8000 as *mut u16}
    }

    pub fn putchxy(&self, ch: char, x: isize, y: isize)
    {
        unsafe {
            *self.screen.offset(y * 80 + x) = ch as u16 | WHITE_ON_BLACK;
        }
    }

    pub fn clear(&self)
    {
        unsafe {
            memset(self.screen as *mut u8, 0, 80*25);
        }
    }

    pub fn scrolldown(&self)
    {
        unsafe {
            memmove(self.screen as *mut u8, self.screen.offset(80) as *mut u8, 25*80*2);
            memset(self.screen.offset(80*25) as *mut u8, 0, 80 * 2);
        }
    }

    pub fn putch(self: &mut Display, ch: char)
    {
        match ch {
            '\n'   => { 
                self.y += 1;
                self.x = 0;
            },
            '\t'   => { (self.x & (!8)) + 8;},
            '\x08' => {
                if self.x > 0 {
                    self.x-=1;
                    self.putch(' ');
                    self.x-=1;
                }
            },
            _      => { self.putchxy(ch, self.x, self.y); self.x += 1; }
        }
        if self.x >= 80 {
            self.x=0;
            self.y += 1;
        }
        if self.y > 25 {
            self.scrolldown();
            self.y = 25;
        }
    }

}

impl fmt::Writer for Display
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {

        for c in s.chars() {
            self.putch(c);
        }
        Ok( () )
    }
}

macro_rules! print {
    ( $($arg:tt)* ) => ({
        use core::fmt::Writer;
        unsafe {
            let _ = write!(&mut ::vga::mainscreen, $($arg)*);
        }
    })
}

