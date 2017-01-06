global long_mode_start

section .text
bits 64
long_mode_start:
    ; print `OKAY` to screen
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    mov rax, 0x2f4e2f4f2f4c2f20
    mov qword [0xb8008], rax
    mov rax, 0x2f002f002f002f47
    mov qword [0xb8016], rax
    hlt
