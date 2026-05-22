; EXERCISE 13: INC and DEC
; INC adds 1 to a register. DEC subtracts 1.
; They are shorter encodings than "add ax, 1" or "sub ax, 1".
;
; Start with AX = 0x000A.
; Increment AX three times, then decrement it once.
; AX should equal 0x000C.
;
; ASSERT_REG: AX == 0x000C
global _start
section .text
_start:
    mov ax, 0x000A
    
    ; Write your code here:
    ; I AM NOT DONE
    
    hlt
