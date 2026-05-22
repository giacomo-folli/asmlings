; EXERCISE 12: Shift Right — Divide by Powers of Two
; SHR (Shift Right) shifts bits toward the LSB, filling with zeros on the left.
; Each right shift by 1 is equivalent to an unsigned division by 2.
;
; Load BX with 0x0080, then shift it right by 3.
; BX should equal 0x0010.
;
; ASSERT_REG: BX == 0x0010
global _start
section .text
_start:
    ; Write your code here:

    hlt
