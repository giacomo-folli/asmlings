; EXERCISE 15: Unsigned Divide (DIV)
; DIV divides the 32-bit value DX:AX by the given operand.
; After division: AX = quotient, DX = remainder.
;
; Divide 0x0063 (99 decimal) by 0x000A (10 decimal).
; AX (quotient) should be 0x0009 (9).
; DX (remainder) should be 0x0009 (9).
;
; Hint: zero out DX before dividing (DX:AX is the dividend).
;
; ASSERT_REG: AX == 0x0009
; ASSERT_REG: DX == 0x0009
global _start
section .text
_start:
    ; Write your code here:

    hlt
