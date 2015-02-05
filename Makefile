OBJ=main.o boot.o int.o

CFLAGS=-std=gnu99 -Wall -Wextra -pedantic -O0 -m32

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

k.elf: ${OBJ}
	ld -T link.ld -m elf_i386 ${OBJ} -o k.elf


