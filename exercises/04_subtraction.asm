; EXERCISE 4: Subtraction
; Set CX to 0x0050, then subtract 0x000F from it.
; The result in CX should be 0x0041.
;
; ASSERT_REG: CX == 0x0041
global _start
section .text
_start:
    ; Write your code here:

    mov cx, 0x0050
    sub cx, 0x000F

    hlt 