; EXERCISE 20: Writing To Memory
; You can also use MOV to store a register's value into memory.
; Syntax:  mov [label], ax   writes AX into the 16-bit word at that label.
;
; The variable `result` starts at 0x0000.
; Compute 0x0007 + 0x0008 in AX, then store AX into `result`.
; AX and the word at `result` should both equal 0x000F.
;
; ASSERT_REG: AX == 0x000F
; ASSERT_MEM: result == 0x000F
global _start
section .text
_start:
    ; Write your code here:
    ; I AM NOT DONE
    
    hlt

section .data
result: dw 0x0000
