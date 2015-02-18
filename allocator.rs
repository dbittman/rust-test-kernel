extern "C" {
    pub static mut end: u32;
}

static mut allocator_end: u32 = 0;

#[lang="exchange_malloc"]
unsafe fn kmalloc(size: usize, align: usize) -> *mut u8 {
    let aligned_size: u32 = ((size & !(align-1)) + align) as u32;
    let ret = allocator_end;
    allocator_end += aligned_size;
    return ret as *mut u8;
}

#[allow(unused_variables)]
#[lang="exchange_free"]
unsafe fn kfree(ptr: *mut u8, old_size: usize, align: usize) {
    /* LOL */
}

pub fn init()
{
    unsafe {
        allocator_end = ::core::mem::transmute(&end);
    }
}

