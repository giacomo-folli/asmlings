; EXERCISE 24: XCHG — Swap Without a Temporary
; XCHG swaps the contents of two registers atomically.
; It is equivalent to a three-instruction swap but fits in a single opcode.
;
; AX starts as 0x1111 and BX starts as 0x2222.
; Use XCHG to swap them.
; AX should equal 0x2222 and BX should equal 0x1111.

global _start
section .text
_start:
    mov ax, 0x1111
    mov bx, 0x2222

    ; Write your code here:
    ; I AM NOT DONE
    
    hlt
