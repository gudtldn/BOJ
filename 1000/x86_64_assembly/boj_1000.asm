; https://www.acmicpc.net/problem/1000
section .data
    input: db "%d %d", 0x0
    output: db "%d", 0xA, 0x0

section .bss
    a: resd 1
    b: resd 1

section .text
    global main
    extern scanf
    extern printf

main:
    ; 스택 프레임 설정
    push rbp
    mov rbp, rsp

    ; scanf("%d %d", &a, &b);
    mov rdi, input
    mov rsi, a
    mov rdx, b
    mov rax, 0
    call scanf

    ; a += b;
    mov rax, [a]
    add rax, [b]

    ; printf("%d", a);
    mov rdi, output
    mov rsi, rax
    mov rax, 0
    call printf

    ; 스택 프레임 되돌리기
    pop rbp
    xor rax, rax
    ret
