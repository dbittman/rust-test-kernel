
pub unsafe fn outportb(port: u16, val: u8)
{
    asm!("outb %al, %dx" : : "{dx}"(port), "{al}"(val));
}

pub unsafe fn inportb(port: u16) -> u8
{
    let ret: u8;
    asm!("inb %dx, %al" : "={ax}"(ret): "{dx}"(port));
    ret
}


