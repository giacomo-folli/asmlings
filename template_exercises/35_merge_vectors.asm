; EXERCISE 35: Merge Vectors
;
; You have two arrays of 16-bit signed integers with the same number of elements:
;     vett1: 5, -2, 9, 0, -7, 3
;     vett2: 1, 4, -3, 8, 2, -6
;
; Write a program that produces a third array, vett3, by iterating
; over vett1 from left to right and vett2 from right to left
; AT THE SAME TIME, alternating sum and difference between the corresponding elements:
;
;   vett3[0] = vett1[0] + vett2[last]
;   vett3[1] = vett1[1] - vett2[second_to_last]
;   vett3[2] = vett1[2] + vett2[...]
;   ... and so on, alternating + and -.
;
; Once vett3 is calculated, sum ALL its negative elements
; (ignore positive or zero ones) and store that total in the AX register.
;
; Constraints:
;   - The calculation of vett3 must happen in a separate function
;     (call it what you want), invoked with `call` from _start.
;   - The addresses of the three arrays and the number of elements N must
;     be passed to the function ON THE STACK (push before the call), not via registers.
;   - The solution must work for any N.
;
; With the example data above, the correct result in AX should be -8 
; (which is 0xFFF8 in 16-bit hex).
;
; Run `asmlings hint 33_merge_vectors` if you get stuck.

global _start
section .text

_start:
    ; TODO: prepare parameters on the stack and call the function
    ; that fills vett3, then calculate the sum of negative numbers and put it in AX.
    
    ; I AM NOT DONE

    hlt

section .data
    vett1:  dw 5, -2, 9, 0, -7, 3
    vett2:  dw 1, 4, -3, 8, 2, -6
    n_elem: dw 6

section .bss
    vett3:  resw 6
