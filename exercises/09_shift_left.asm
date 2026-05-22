; EXERCISE 9: Shift Left — Multiply by Powers of Two
; SHL (Shift Left) shifts bits to the left, filling with zeros on the right.
; Each left shift by 1 is equivalent to multiplying by 2.
;
; Load AX with 0x0003, then shift it left by 4.
; AX should equal 0x0030.
;
; ASSERT_REG: AX == 0x0030
global _start
section .text
_start:
    ; I AM NOT DONE
    
    mov ax, 0x0003
    shl ax, 4

    hlt