; EXERCISE 3: Addition
; Start with BX = 0x0010, then add 0x0005 to it.
; The result in BX should be 0x0015.
;
; ASSERT_REG: BX == 0x0015
global _start
section .text
_start:
    ; Write your code here:
    ; I AM NOT DONE
    
    mov bx, 0x0010
    add bx, 0x0005
    
    hlt