; EXERCISE 2: Register Halves
; The 16-bit registers can be accessed as two 8-bit halves. 
; AX is made of AH (high byte) and AL (low byte).
;
; Set AH to 0xAB and AL to 0xCD so that the full AX register equals 0xABCD.
;
; ASSERT_REG: AX == 0xABCD

global _start
section .text
_start:
    ; I AM NOT DONE
    ; Write your code here:

    mov ah, 0xAB
    mov al, 0xCD
    
    hlt
