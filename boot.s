section .boot
MBOOT_PAGE_ALIGN	equ 1<<0	; Load kernel and modules on a page boundary
MBOOT_MEM_INFO		equ 1<<1	; Provide kernel with memory info
MBOOT_O_INFO		equ 1<<2	
MBOOT_HEADER_MAGIC	equ 0x1BADB002	; Multiboot Magic value
MBOOT_HEADER_FLAGS	equ MBOOT_PAGE_ALIGN | MBOOT_MEM_INFO | MBOOT_O_INFO
MBOOT_CHECKSUM		equ -(MBOOT_HEADER_MAGIC + MBOOT_HEADER_FLAGS)

[BITS 32]                       ; All instructions should be 32-bit.

[GLOBAL mboot]                  ; Make 'mboot' accessible from C.
[EXTERN code]                   ; Start of the '.text' section.
[EXTERN bss]                    ; Start of the .bss section.
[EXTERN end]                    ; End of the last loadable section.

section .multiboot

mboot:
    dd  MBOOT_HEADER_MAGIC      ; GRUB will search for this value on each
                                ; 4-byte boundary in your kernel file
    dd  MBOOT_HEADER_FLAGS      ; How GRUB should load your file / settings
    dd  MBOOT_CHECKSUM          ; To ensure that the above values are correct
    
    dd  mboot                   ; Location of this descriptor
    dd  code                    ; Start of kernel '.text' (code) section.
    dd  bss                     ; End of kernel '.data' section.
    dd  end                     ; End of kernel.
    dd  start                   ; Kernel entry point (initial EIP).

section .text
[GLOBAL start]                  ; Kernel entry point.
[GLOBAL _start]                 ; Kernel entry point.
[EXTERN kmain]                  ; This is the entry point of our C code
STACKSIZE equ 0x4000            ; that's 16k.

start:
   mov esp, stack+STACKSIZE-4   ; set up the stack
   push esp                     ; kernel expects and esp value in the arguments
   push ebx                     ; pass Multiboot info structure
   cli                          ; start with ints disabled
   call  kmain                  ; call kernel proper

   ; force a reset
   cli
   lgdt [0]
   lidt [0]
   sti
   int 3
   
hang:
   hlt                          ; halt machine should kernel return
   jmp   hang                   ; course, this wont happen

align 32
idtp:
	dw 1
	dd 1

[GLOBAL load_idt]
extern idt
load_idt:
	xchg bx, bx
	push ebp
	mov ebp, esp
	mov eax, [ebp+12]
	mov ecx, [ebp+8]
	mov word [idtp], ax
	mov dword [idtp+2], ecx

	lidt [idtp]

	pop ebp
	ret

[GLOBAL reload_segments]
reload_segments:
	push ebp
	mov ebp, esp

	jmp 0x08:__RS_reload_cs
	__RS_reload_cs:
	mov ax, 0x10
	mov ds, ax
	mov ss, ax
	mov fs, ax
	mov es, ax
	mov gs, ax

	pop ebp
	ret

section .bss
align 32
stack:
   resb STACKSIZE               ; reserve 16k stack on a quadword boundary

