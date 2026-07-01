; EXERCISE 34: Count the Set Bits — Popcount (Third Long Program)
; Count how many bits are 1 in a 16-bit value and store the count in BX.
; This operation is known as "population count" or "popcount".
;
; Input value: 0xA5F3  (binary: 1010 0101 1111 0011)
; Expected number of set bits: 9
; BX should equal 0x0009.
;
; Suggested algorithm (shift-and-test):
;   • Use CX = 16 as the loop counter (one iteration per bit).
;   • Each iteration: use SHR (or ROR) on AX to shift a bit into the Carry Flag.
;     With SHR, bit 0 shifts into CF, so use:
;         shr ax, 1
;         adc bx, 0    ; adds CF to BX — clever one-liner!
;   • Repeat 16 times with LOOP.
;
; Alternative: use TEST + JZ to check bit 0 each round, then SHR by 1.

global _start
section .text
_start:
    mov ax, 0xA5F3      ; value to count bits in
    xor bx, bx          ; BX = 0  (bit counter)
    mov cx, 16          ; 16 bits to process

    ; Write your loop here.
    ; Hint: "shr ax, 1" shifts bit 0 of AX into CF.
    ;       "adc bx, 0" adds CF to BX without any extra branch.

    ; I AM NOT DONE    

    hlt
