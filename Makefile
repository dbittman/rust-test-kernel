AOBJ=boot.o int.o
ROBJ=kernel.o

OBJ=$(ROBJ) $(AOBJ)

CFLAGS=-std=gnu99 -Wall -Wextra -pedantic -O0 -m32

RFLAGS := --cfg arch__x86 --target=target.json -C opt-level=0 -C no-stack-check -g


all: k.elf
	sudo sh tools/open_hdimage.sh
	sudo cp k.elf /mnt
	sudo sh tools/close_hdimage.sh

-include kernel.d

clean:
	rm *.o k.elf libcore.rlib *.d

test:
	qemu-system-i386 hd.img

.s.o:
	nasm -felf32 $<

.c.o:
	gcc $(CFLAGS) -c $<

%.o : %.rs Makefile libcore.rlib
	rustc ${RFLAGS} --emit=obj,dep-info $< --extern core=libcore.rlib

libcore.rlib: ../rust/src/libcore/lib.rs
	rustc ${RFLAGS} --crate-type=lib,staticlib --emit=link,dep-info ../rust/src/libcore/lib.rs

k.elf: $(OBJ)
	ld -static -T link.ld -m elf_i386 --gc-sections -z max-page-size=0x1000 $(OBJ) libcore.a -o k.elf

