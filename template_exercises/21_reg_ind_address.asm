; EXERCISE 21: Register-Indirect Addressing (BX as a Pointer)
; On the 8086, BX (and SI/DI) can be used as a memory pointer.
; Syntax:  mov ax, [bx]   reads the word at the address stored in BX.
;
; The label `treasure` holds the value 0xF00D.
; Load the ADDRESS of `treasure` into BX using:
;     mov bx, treasure
; Then load the VALUE at that address into AX using indirect addressing.
; AX should equal 0xF00D.

global _start
section .text
_start:
    ; Write your code here:
    ; I AM NOT DONE
    
    hlt

section .data
treasure: dw 0xF00D
