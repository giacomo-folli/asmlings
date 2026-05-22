; EXERCISE 19: Reading From Memory
; The MOV instruction can load a value from a memory address into a register.
; Syntax:  mov ax, [label]   reads the 16-bit word stored at that label.
;
; A word (0xDEAD) is already stored in the data section below.
; Load it into AX.
;
; ASSERT_REG: AX == 0xDEAD
global _start
section .text
_start:
    ; Write your code here (load the word at `my_value` into AX):

    hlt

section .data
my_value: dw 0xDEAD
