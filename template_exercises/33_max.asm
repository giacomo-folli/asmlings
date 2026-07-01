; EXERCISE 33: Find the Maximum (Second Long Program)
; Scan an array of five unsigned 16-bit words and leave the largest value in AX.
;
; The array is: 0x000A, 0x002F, 0x0017, 0x003E, 0x0008
; The maximum is 0x003E.
;
; Suggested approach:
;   • Use SI to walk through the array (add 2 after each element).
;   • Use CX as a counter (5 iterations).
;   • Keep the running maximum in AX.
;   • Use CMP + JA (unsigned "jump if above") to update the maximum.

global _start
section .text
_start:
    mov si, numbers         ; SI = pointer to start of array
    mov cx, 5               ; CX = number of elements
    mov ax, 0x0000          ; AX = running maximum (start at 0)

    ; Write your loop here:
    ; I AM NOT DONE    


    hlt

section .data
numbers: dw 0x000A, 0x002F, 0x0017, 0x003E, 0x0008
