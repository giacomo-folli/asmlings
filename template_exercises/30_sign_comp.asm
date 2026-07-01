; EXERCISE 30: Signed Comparisons — JL and JG
; JL (Jump if Less)    uses the Sign and Overflow flags for SIGNED comparison.
; JG (Jump if Greater) is the signed equivalent of JA.
; Compare with JB/JA which are unsigned (Carry-based).
;
; AX = 0xFFFE which in two's complement is -2 (a negative number).
; Compare AX to 0x0000.
; Because -2 < 0 (signed), jump to `is_negative` and set BX = 0xFFFF.
; Otherwise jump to `is_positive` and set BX = 0x0001.

global _start
section .text
_start:
    mov ax, 0xFFFE

    ; Write your code here:
    ; I AM NOT DONE
    
is_negative:
    mov bx, 0xFFFF
    hlt

is_positive:
    mov bx, 0x0001
    hlt
