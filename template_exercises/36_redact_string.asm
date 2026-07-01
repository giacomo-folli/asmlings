; EXERCISE 36: Redact String
;
; The program must:
;
;   1. Process a string (input_str) already defined in memory.
;      It's a standard null-terminated string (ends with '\0').
;
;   2. Use a fixed array of indices to "redact":
;
;        indices: 1, 4, 4, 9, 50
;
;      Each number is a position (0 = first character) in the
;      ORIGINAL string. The character at that position must be
;      removed from the final output.
;
;   3. IMPORTANT: indices always refer to the original string,
;      not the modified one. Removing a character doesn't change
;      the position of the following characters. A duplicate index
;      must not cause errors. An index that exceeds the string length
;      must be simply ignored.
;
;   4. Write the resulting string (without the redacted characters)
;      into the `out_buf` buffer in memory. The result must also be
;      null-terminated!
;
; Example: for the input string
;
;      Hello world!
;
; (positions: H=0 e=1 l=2 l=3 o=4 space=5 w=6 o=7 r=8 l=9 d=10 !=11)
; removing positions 1, 4, 4, 9, 50 (50 out of bounds -> ignored,
; 4 duplicate -> removed only once), the result in out_buf must be:
;
;      Hll word!
;
; Constraints:
;   - The redaction must be done by scanning the original string
;     ONCE and copying only the characters to keep into out_buf.
;   - This logic must be in a function called with `call`, to which
;     you pass via stack: the address of the input string, the address
;     of the indices array, and the number of indices.
;
; Run `asmlings hint 34_redact_string` if you get stuck.

global _start
section .text

_start:
    ; TODO: prepare parameters on the stack and call the function
    ; that processes input_str and produces out_buf.
    
    ; I AM NOT DONE

    hlt

section .data
    input_str: db "Hello world!", 0
    indici:    db 1, 4, 4, 9, 50
    n_indici:  dw 5

section .bss
    out_buf:   resb 256
