ENTRY(start)
OUTPUT_FORMAT(elf32-i386)
SECTIONS
{
	. = 0x100000;
	. += SIZEOF_HEADERS;
	.multiboot : AT(ADDR(.multiboot)) {
		KEEP( *(.multiboot) )
	}
    .text :
    {
		code = .; _code = .; __code = .;
		kernel_start = .;
		*(.text .text.*)
		. = ALIGN(4096);
    }

    .data :
    {
        data = .; _data = .; __data = .;
		*(.padata .padata.*)
        *(.data .data.*)
        *(.rodata .rodata.*)
        . = ALIGN(4096);
    }

    .bss :
    {
        bss = .; _bss = .; __bss = .;
        *(.bss .bss.*)
        . = ALIGN(4096);
    }
    end = .; _end = .; __end = .;
}

