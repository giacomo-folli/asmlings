; EXERCISE 23: CALL and RET — Subroutines
; CALL pushes the return address onto the stack and jumps to a label.
; RET  pops that address and returns execution to the caller.
;
; A subroutine `double` is provided below — it doubles AX.
; Load AX with 0x0009, call `double`, and the result in AX should be 0x0012.
;
; ASSERT_REG: AX == 0x0012
global _start
section .text
_start:
    mov ax, 0x0009
    
    ; Write your code here:
    ; I AM NOT DONE
    
    hlt

double:
    shl ax, 1
    ret
