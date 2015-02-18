mod x86_tables;
pub fn initialize_processor()
{
    unsafe {
        x86_tables::gdt_init();
        x86_tables::idt_init();
        x86_tables::pic_init();
    }
}

