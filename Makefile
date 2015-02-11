OBJ=boot.o int.o libcore.rlib kernel.o

CFLAGS=-std=gnu99 -Wall -Wextra -pedantic -O0 -m32

RFLAGS := --cfg arch__x86 --target=target.json -O

all: k.elf
	sudo sh tools/open_hdimage.sh
	sudo cp k.elf /mnt
	sudo sh tools/close_hdimage.sh

clean:
	rm *.o k.elf

test:
	qemu-system-i386 hd.img

.s.o:
	nasm -felf32 $<

.c.o:
	gcc $(CFLAGS) -c $<

kernel.o: kernel.rs libcore.rlib Makefile
	rustc ${RFLAGS} --emit=obj,dep-info $< --extern core=libcore.rlib

libcore.rlib: ../rust/src/libcore/lib.rs Makefile
	rustc ${RFLAGS} --crate-type=lib --emit=link,dep-info ../rust/src/libcore/lib.rs

k.elf: ${OBJ}
	ld -static -T link.ld -m elf_i386 --gc-sections -z max-page-size=0x1000 ${OBJ} -o k.elf

