; https://www.acmicpc.net/problem/1001
section .data
    input db "%d %d", 0x0
    output db "%d", 0xA, 0x0

section .bss
    a resd 1
    b resd 1

section .text
    global main
    extern printf
    extern scanf

main:
    push rbp
    mov rbp, rsp

    ; 입력값 받기
    mov rdi, input
    mov rsi, a
    mov rdx, b
    mov rax, 0
    call scanf

    ; a -= b
    mov rax, [a]
    sub rax, [b]

    ; 결과 출력
    mov rdi, output
    mov rsi, rax
    mov rax, 0
    call printf

    pop rbp
    xor rax, rax
    ret
