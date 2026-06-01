; EXERCISE 22: Array Traversal with SI
; SI (Source Index) is commonly used to walk through arrays.
; "[si]" reads the word at the address in SI, and "add si, 2" advances to the
; next word (each word is 2 bytes).
;
; An array of three words is defined below: 0x0001, 0x0002, 0x0003.
; Load each element into AX one at a time (using [si], then advance SI),
; accumulating a running sum in BX.
; BX should equal 0x0006.

global _start
section .text
_start:
    mov si, numbers
    xor bx, bx
    
    ; Write your code here:
    ; I AM NOT DONE

    hlt

section .data
numbers: dw 0x0001, 0x0002, 0x0003
