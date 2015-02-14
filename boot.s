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

[GLOBAL memcpy]
memcpy:
	push ebp
	; new stack frame
	mov ebp, esp

	; save regs
	push esi
	push edi

	; destination -> edi
	mov edi, [ebp + 8]
	mov eax, edi ; save destination for return

	; source -> esi, count -> ecx
	mov esi, [ebp + 12]
	mov ecx, [ebp + 16]

	; do the copy (isn't x86 great?)
	rep movsb
	
	; restore
	pop edi
	pop esi
	pop ebp

	ret

section .bss
align 32
stack:
   resb STACKSIZE               ; reserve 16k stack on a quadword boundary
