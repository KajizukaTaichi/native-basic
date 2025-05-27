
print:
    lodsb
    or al, al
    jz print_done
    mov ah, 0x0E
    int 0x10
    jmp print
print_done:
    ret
