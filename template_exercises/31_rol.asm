; EXERCISE 31: ROL — Rotate Left
; ROL rotates bits left: the MSB that falls off the top wraps around to the LSB.
; Unlike SHL, no bits are lost — they cycle around.
;
; Load AX with 0x8001 (binary: 1000 0000 0000 0001).
; Rotate left by 1.
; The MSB (1) wraps to the LSB position, and every other bit shifts left.
; AX should equal 0x0003 (binary: 0000 0000 0000 0011).

global _start
section .text
_start:
    ; Write your code here:
    ; I AM NOT DONE
    
    hlt
