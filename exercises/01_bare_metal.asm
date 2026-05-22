; EXERCISE 1: The Bare Metal
; The 8086 CPU has four main general-purpose 16-bit registers: AX, BX, CX, DX.
; Move the value 0x1337 into the AX register.
;
; ASSERT_REG: AX == 0x1337
global _start
section .text
_start:
    ; Write your code here:
    mov ax, 0x1337
    
    hlt