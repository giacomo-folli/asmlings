; EXERCISE 10: The Stack — PUSH and POP
; The stack is a region of memory that works Last-In, First-Out (LIFO).
; PUSH decrements SP by 2 and writes a word to [SS:SP].
; POP  reads a word from [SS:SP] and increments SP by 2.
;
; Push 0xCAFE onto the stack (via AX), then pop it into BX.
; BX should equal 0xCAFE.

global _start
section .text
_start:
    ; Write your code here:
    ; I AM NOT DONE

    hlt