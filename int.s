isr_handler:
	cli
	pusha

	mov ax, ds
	push ds
	
	mov ax, 0x10
	mov ds, ax
	mov ss, ax
	mov gs, ax
	mov fs, ax

	; call c code

	; pop ebp
	pop ax
	mov ds, ax
	mov ss, ax
	mov gs, ax
	mov fs, ax

	popa
	sti
	iret

