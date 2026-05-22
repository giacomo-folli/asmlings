; EXERCISE 8: XOR — The Zero Trick
; XOR-ing a register with itself always yields zero.
; This is a classic 8086 idiom — shorter and faster than "mov ax, 0".
;
; Zero out both CX and DX without using the MOV instruction.
;
; ASSERT_REG: CX == 0x0000
; ASSERT_REG: DX == 0x0000
global _start
section .text
_start:
    ; I AM NOT DONE

    mov ax, 0x0045

    mov cx, ax
    mov dx, ax

    xor cx, cx
    xor dx, dx

    hlt