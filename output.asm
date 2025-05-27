[ORG 0x7C00]
[BITS 16]
	mov si, str_literal_0
	call print
	mov si, str_literal_1
	call print
	jmp $

print:
    lodsb
    or al, al
    jz print_done
    mov ah, 0x0E
    int 0x10
    jmp print
print_done:
    mov al, 0x0D
    int 0x10
    mov al, 0x0A
    int 0x10
    ret

str_literal_0 db "Hello, world!", 0
str_literal_1 db "from Native Basic", 0

times 510 - ($ - $$) db 0
dw 0xAA55

