[ORG 0x7C00]
[BITS 16]
	mov si, str_literal_0
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
    ret

str_literal_0 db "Hello", 0

times 510 - ($ - $$) db 0
dw 0xAA55

