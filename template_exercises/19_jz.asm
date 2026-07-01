; EXERCISE 19: Jump if Zero (JZ)
; JZ (Jump if Zero) jumps to a label if the Zero Flag (ZF) is set.
; ZF is set by instructions like CMP, SUB, DEC, etc. if the result is zero.
;
; Task:
; Check if AX is zero. If AX is zero, jump to .is_zero to set BX to 0x0001.
; Otherwise, set BX to 0x0002.
;
; Start: AX is 0x0000.

global _start
section .text
_start:
    ; AX is initialized during test setup.
    
    ; Write your code here:
    ; I AM NOT DONE

    mov bx, 0x0002
    jmp .done

.is_zero:
    mov bx, 0x0001

.done:
    hlt
