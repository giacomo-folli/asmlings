; EXERCISE 17: CMP and Conditional Jump
; CMP subtracts its second operand from the first WITHOUT storing the result.
; It only updates the FLAGS register.
; JE  (Jump if Equal)   jumps when the Zero Flag (ZF) is set.
; JNE (Jump if Not Equal) jumps when ZF is clear.
;
; AX starts at 0x000A.
; If AX equals 0x000A, set BX to 0x0001. Otherwise set BX to 0x0002.
; Since AX == 0x000A, BX should end up as 0x0001.
;
; ASSERT_REG: BX == 0x0001
global _start
section .text
_start:
    mov ax, 0x000A
    ; Write your code here (use CMP and JE/JNE):

    hlt
