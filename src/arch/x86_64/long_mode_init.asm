global long_mode_start

section .text
bits 64
long_mode_start:
    ; Call the rust main
    extern rust_main
    call rust_main

    ; Print `OKAYLONG` to screen
    mov rax, 0x2f472f4e2f4f2f4c
    mov qword [0xb8000], rax
    mov rax, 0x2f452f442f4f2f4d
    mov qword [0xb8008], rax
    hlt
