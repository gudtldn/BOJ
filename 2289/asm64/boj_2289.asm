section .data
    duck: db '<(o )___', 0x0a,
          db ' ( \_> /', 0x0a,
          db '  "~~~"', 0x0a
    duck_len: equ $-duck

section .text
    global main

main:
    mov eax, 0x4
    mov ebx, 1
    mov ecx, duck
    mov edx, duck_len
    int 0x80

    mov eax, 0x0
    ret
