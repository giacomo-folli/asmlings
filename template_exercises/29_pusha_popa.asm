; EXERCISE 29: PUSHA and POPA — Save and Restore All Registers
; PUSHA pushes AX, CX, DX, BX, SP (before push), BP, SI, DI onto the stack.
; POPA  restores them in reverse order (it ignores the saved SP value).
; This is used to preserve register state across a subroutine call.
;
; 1. Load AX=0x0001, BX=0x0002, CX=0x0003.
; 2. Call `clobber` (it destroys all registers).
; 3. POPA will restore AX, BX, CX to their original values.
;
; After POPA: AX == 0x0001, BX == 0x0002, CX == 0x0003.

global _start
section .text
_start:
    mov ax, 0x0001
    mov bx, 0x0002
    mov cx, 0x0003
    pusha
    call clobber

    ; Write your code here:
    ; I AM NOT DONE
    
    hlt

clobber:
    xor ax, ax
    xor bx, bx
    xor cx, cx
    ret
