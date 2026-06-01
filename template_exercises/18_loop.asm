; EXERCISE 18: The LOOP Instruction
; LOOP decrements CX and jumps to the label if CX != 0.
; It is shorthand for: DEC CX + JNZ label.
;
; Use LOOP to add 0x0003 to AX exactly 4 times.
; Start: AX = 0x0000, CX = 4.
; AX should equal 0x000C (12 decimal) after the loop.

global _start
section .text
_start:
    xor ax, ax
    mov cx, 4
    
    ; Write your code here:
    ; I AM NOT DONE
    
    hlt
