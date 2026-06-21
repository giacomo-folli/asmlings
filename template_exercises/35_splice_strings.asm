; EXERCISE 35: Splice Strings
;
; You have two C strings (terminated by '\0'):
;
;     s1: "ciao mondo"
;     s2: "buonasera"
;
; Construct a third string s3 taking, in this order:
;
;   1. the first half of s1 (read normally)
;   2. the second half of s2, but written IN REVERSE
;
; "Half" of a string of length L means the first L/2
; characters (first half) and the remaining characters (second half).
; If L is odd, round down (e.g., L=9 -> first half = 4 characters,
; the second half will therefore have 5).
;
; Example with the data above:
;   s1 = "ciao mondo" (10 chars) -> first half (5) = "ciao "
;   s2 = "buonasera" (9 chars)   -> second half (5) = "asera"
;        written in reverse      = "aresa"
;
;   resulting s3 = "ciao aresa"
;
; Constraints:
;   - Calculate the length of the strings with a dedicated function
;     (write it yourself, iterating until you find the '\0').
;   - The copy into s3 must happen in ONE function called
;     CopiaStringhe, to which you pass via STACK the addresses of s1,
;     s2, and s3 (push before the call, no implicit registers).
;   - s3 must remain a valid C string (terminated by '\0') at the end.
;
; Run `asmlings hint 35_splice_strings` if you get stuck.

global _start
section .text

_start:
    ; I AM NOT DONE

    hlt

CopiaStringhe:
    ; TODO: implement the logic to splice the strings.
    ; You can call CalcolaLunghezza from here to get string lengths.


CalcolaLunghezza:
    ; TODO: implement the string length calculation.


section .data
    s1: db 'ciao mondo', 0
    s2: db 'buonasera', 0
    s3: resb 64