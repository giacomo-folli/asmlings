; EXERCISE 25: The Carry Flag and ADC
; ADD sets the Carry Flag (CF) when the result overflows 16 bits.
; ADC (Add with Carry) adds two operands PLUS the current CF.
; Together, ADD + ADC allow you to add 32-bit numbers stored in two registers.
;
; Add the two 32-bit numbers:
;   A = 0x0001_FFFF   (DX=0x0001, AX=0xFFFF)
;   B = 0x0000_0001   (CX=0x0000, BX=0x0001)
;
; Low word:  AX + BX  (may produce a carry)
; High word: DX + CX + CF  (use ADC)
;
; Expected result: 0x0002_0000
;   AX == 0x0000, DX == 0x0002
;
; ASSERT_REG: AX == 0x0000
; ASSERT_REG: DX == 0x0002
global _start
section .text
_start:
    mov dx, 0x0001
    mov ax, 0xFFFF
    mov cx, 0x0000
    mov bx, 0x0001

    ; Write your code here:
    ; I AM NOT DONE
    
    hlt
