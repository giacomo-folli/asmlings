; EXERCISE 7: Bitwise OR — Setting Bits
; The OR instruction sets bits. If either operand has a 1, the result has a 1.
;;
; Start with BX = 0x0F00 and mask it
; BX should equal 0x0FF0.
;
; ASSERT_REG: BX == 0x0FF0
global _start
section .text
_start:
    ; Write your code here:

    mov bx, 0x0F00
    or  bx, 0x00F0

    hlt