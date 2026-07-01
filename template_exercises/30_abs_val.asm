; EXERCISE 30: Absolute Value (First Long Program)
; Write a short program that computes the absolute value of a signed 16-bit
; number stored in AX.
;
; Strategy (two's complement absolute value):
;   1. Check if AX is negative (MSB set, or compare < 0).
;   2. If negative: negate AX with NEG.
;   3. If non-negative: leave AX unchanged.
;
; Two test cases are set up. The subroutine `abs_val` should handle both.
; After the routine, AX must hold the positive value.
;
; Test case A: AX = 0xFFF6 (-10 signed) → result 0x000A
; Test case B: AX = 0x0007 (+7  signed) → result 0x0007
;
; The harness tests case A.  Make sure your logic handles both conceptually.

global _start
section .text
_start:
    mov ax, 0xFFF6      ; -10 in two's complement
    call abs_val
    hlt

; -----------------------------------------------
; abs_val: compute |AX|, result in AX
; Clobbers: FLAGS only
; -----------------------------------------------
abs_val:
    ; Write your code here:
    ; Hint: compare AX to 0, then conditionally NEG it.
    
    ; I AM NOT DONE    

    ret
