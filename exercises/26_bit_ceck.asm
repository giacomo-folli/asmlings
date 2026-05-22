; EXERCISE 26: TEST — Non-Destructive Bit Check
; TEST performs a bitwise AND but ONLY updates FLAGS; it does not store the result.
; It is the read-only counterpart of AND, used to check individual bits.
; If the tested bit is 0 → ZF=1. If non-zero → ZF=0.
;
; AX = 0x0005 (binary: 0000 0000 0000 0101).
; Use TEST to check bit 0 (the LSB).
; Because bit 0 IS set, jump to `bit_set` and place 0x0001 in BX.
; If the bit were clear you would jump to `bit_clear` and put 0x0000 in BX.
;
; ASSERT_REG: BX == 0x0001
global _start
section .text
_start:
    mov ax, 0x0005
    ; Write your code here (TEST + JNZ/JZ):

bit_set:
    mov bx, 0x0001
    hlt

bit_clear:
    mov bx, 0x0000
    hlt
