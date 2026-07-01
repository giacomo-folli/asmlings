; EXERCISE 18: Unconditional Jumps
; The JMP instruction jumps unconditionally to a specified label.
; This allows you to skip over sections of code or redirect execution.
;
; Task: Modify AX so that it ends up containing 0x1111.
; Do this by using JMP to skip the instruction that sets AX to 0x2222.
;
; Start: AX is set to 0x1111.
; If you don't jump, AX will be overwritten with 0x2222.

global _start
section .text
_start:
    mov ax, 0x1111

    ; Write your code here:
    ; I AM NOT DONE

    mov ax, 0x2222

.target:
    hlt
