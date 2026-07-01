pub fn get_hint(exercise_name: &str) -> Option<&'static str> {
    match exercise_name {
        "01_bare_metal" => Some(
            "Use the `mov` instruction to load the value `0x1337` into the `ax` register.\n\
             Example: `mov ax, 0x1337`"
        ),
        "02_halves" => Some(
            "To make AX equal 0xABCD, you can load its high byte (AH) with 0xAB and low byte (AL) with 0xCD.\n\
             Example:\n\
               mov ah, 0xAB\n\
               mov al, 0xCD"
        ),
        "03_addition" => Some(
            "Load 0x0010 into BX using `mov bx, 0x0010`, then add 0x0005 to BX using the `add` instruction.\n\
             Example:\n\
               mov bx, 0x0010\n\
               add bx, 0x0005"
        ),
        "04_subtraction" => Some(
            "Load 0x0050 into CX, then use the `sub` instruction to subtract 0x000F from it.\n\
             Example:\n\
               mov cx, 0x0050\n\
               sub cx, 0x000F"
        ),
        "05_reg_to_reg" => Some(
            "Load 0xBEEF into AX, then copy AX into DX using the `mov` instruction.\n\
             Example:\n\
               mov ax, 0xBEEF\n\
               mov dx, ax"
        ),
        "06_bitwise_and" => Some(
            "To mask out the high byte and keep only the low byte, bitwise AND AX with 0x00FF using the `and` instruction.\n\
             Example:\n\
               mov ax, 0xABCD\n\
               and ax, 0x00FF"
        ),
        "07_bitwise_or" => Some(
            "The bits in the low nibble/byte need to be set. Use the `or` instruction to set the appropriate bits of BX (bitwise OR BX with 0x00F0).\n\
             Example: `or bx, 0x00F0`"
        ),
        "08_xor" => Some(
            "XORing a register with itself always yields 0. Use `xor` on CX and DX.\n\
             Example:\n\
               xor cx, cx\n\
               xor dx, dx"
        ),
        "09_shift_left" => Some(
            "Use the `shl` instruction to shift AX left by 4 bits.\n\
             Example:\n\
               mov ax, 0x0003\n\
               shl ax, 4"
        ),
        "10_stack" => Some(
            "Load 0xCAFE into AX, use `push ax` to push it onto the stack, then use `pop bx` to restore it into BX.\n\
             Example:\n\
               mov ax, 0xCAFE\n\
               push ax\n\
               pop bx"
        ),
        "11_bitwise_not" => Some(
            "Load 0x00FF into AX, then use the `not` instruction to flip every bit of AX.\n\
             Example:\n\
               mov ax, 0x00FF\n\
               not ax"
        ),
        "12_shift_right" => Some(
            "Use the `shr` instruction to shift BX right by 3 bits.\n\
             Example:\n\
               mov bx, 0x0080\n\
               shr bx, 3"
        ),
        "13_inc_dec" => Some(
            "Use the `inc` instruction 3 times and the `dec` instruction once on the `ax` register.\n\
             Example:\n\
               inc ax\n\
               inc ax\n\
               inc ax\n\
               dec ax"
        ),
        "14_mul" => Some(
            "Use `mul` with a 16-bit register operand. For `mul bx`, the result is DX:AX = AX * BX.\n\
             Example:\n\
               mov ax, 0x0005\n\
               mov bx, 0x0006\n\
               mul bx"
        ),
        "15_div" => Some(
            "For 16-bit division, DIV divides DX:AX by the operand. Make sure to zero out DX first so DX:AX equals just AX.\n\
             Example:\n\
               mov ax, 99\n\
               xor dx, dx\n\
               mov bx, 10\n\
               div bx"
        ),
        "16_neg" => Some(
            "Use the `neg` instruction to compute the two's complement negation of AX.\n\
             Example:\n\
               mov ax, 5\n\
               neg ax"
        ),
        "17_cmp" => Some(
            "Use `cmp ax, 0x000A` followed by a conditional jump like `je`. If equal, jump to a label that loads 1 into BX. Otherwise, load 2 into BX and jump to the end.\n\
             Example:\n\
               cmp ax, 0x000A\n\
               je is_equal\n\
               mov bx, 0x0002\n\
               jmp done\n\
             is_equal:\n\
               mov bx, 0x0001\n\
             done:"
        ),
        "18_loop" => Some(
            "The `loop` instruction decrements CX and jumps if CX != 0. Define a label at the start of your loop, perform `add ax, 3` inside, and finish with `loop label`.\n\
             Example:\n\
               add_loop:\n\
                 add ax, 3\n\
                 loop add_loop"
        ),
        "19_read_mem" => Some(
            "Use square brackets around the label name to dereference its memory address and read its value.\n\
             Example: `mov ax, [my_value]`"
        ),
        "20_write_mem" => Some(
            "Compute the sum in AX, then use the `mov [result], ax` instruction to write the 16-bit value into the variable `result` in memory.\n\
             Example:\n\
               mov ax, 7\n\
               add ax, 8\n\
               mov [result], ax"
        ),
        "21_reg_ind_address" => Some(
            "First, load the address of `treasure` into BX using `mov bx, treasure`. Second, dereference BX using `[bx]` to load the value at that address into AX.\n\
             Example:\n\
               mov bx, treasure\n\
               mov ax, [bx]"
        ),
        "22_source_index" => Some(
            "Walk the array using the SI register as a pointer. Read a word with `[si]`, add it to BX, and advance SI by 2 (since each word is 2 bytes).\n\
             Example:\n\
               mov ax, [si]\n\
               add bx, ax\n\
               add si, 2\n\
               ; (repeat or loop for all 3 elements)"
        ),
        "23_subroutines" => Some(
            "Use the `call` instruction to execute the `double` subroutine. The subroutine will return automatically using `ret`.\n\
             Example: `call double`"
        ),
        "24_xchg" => Some(
            "Use the `xchg` instruction to swap the values of AX and BX atomically.\n\
             Example: `xchg ax, bx`"
        ),
        "25_carry_flag" => Some(
            "First, add the lower 16-bit words using `add ax, bx` (which will set the Carry Flag when it overflows). Then, add the higher 16-bit words using `adc dx, cx` to include the Carry Flag.\n\
             Example:\n\
               add ax, bx\n\
               adc dx, cx"
        ),
        "26_bit_check" => Some(
            "Use `test ax, 1` to check the LSB. Then, use `jnz bit_set` to jump if the bit is set (non-zero). Otherwise, jump to `bit_clear`.\n\
             Example:\n\
               test ax, 1\n\
               jnz bit_set\n\
               jmp bit_clear"
        ),
        "27_pusha_popa" => Some(
            "Use the `popa` instruction to pop all general-purpose registers off the stack, restoring them to the values they had when `pusha` was called.\n\
             Example: `popa`"
        ),
        "28_sign_comp" => Some(
            "Use `cmp ax, 0` followed by `jl is_negative` (jump if less, which performs a signed comparison). If the condition isn't met, jump to `is_positive`.\n\
             Example:\n\
               cmp ax, 0\n\
               jl is_negative\n\
               jmp is_positive"
        ),
        "29_rol" => Some(
            "Use the `rol` instruction to rotate AX left by 1 bit.\n\
             Example:\n\
               mov ax, 0x8001\n\
               rol ax, 1"
        ),
        "30_test_1" => Some(
            "Check if AX is negative by comparing it to 0 (`cmp ax, 0`). If it is less than 0, negate it with `neg ax`.\n\
             Example:\n\
               cmp ax, 0\n\
               jge done\n\
               neg ax\n\
             done:\n\
               ret"
        ),
        "31_test_2" => Some(
            "Loop 5 times (using CX). In each iteration, compare AX to the current word at `[si]`. If AX is less, load that word into AX. Then advance SI by 2.\n\
             Example:\n\
               max_loop:\n\
                 cmp ax, [si]\n\
                 jae skip\n\
                 mov ax, [si]\n\
               skip:\n\
                 add si, 2\n\
                 loop max_loop"
        ),
        "32_test_3" => Some(
            "Loop 16 times. In each iteration, shift AX right by 1 to put the lowest bit into the Carry Flag (`shr ax, 1`), then add the Carry Flag to BX using `adc bx, 0`.\n\
             Example:\n\
               count_loop:\n\
                 shr ax, 1\n\
                 adc bx, 0\n\
                 loop count_loop"
        ),
        "33_merge_vectors" => Some(
            "Use two separate index pointers or registers (like SI for vett1 and DI for vett2) and advance them in opposite directions.\n\
             Keep track of an iteration counter (CX) and use a flag or check the lowest bit of the counter to alternate between sum and difference.\n\
             Finally, loop through vett3 and use `test ax, ax` or `cmp ax, 0` followed by `jns` to skip positive numbers when summing."
        ),
        "34_redact_string" => Some(
            "Maintain a counter for the current position in the original string. When reading a character, scan the `indici` array to check if the current position is listed.\n\
             If it is listed, skip writing the character to `out_buf`. If it's not listed, write it and increment the `out_buf` pointer.\n\
             Don't forget to append the null terminator '\\0' to `out_buf` at the very end!"
        ),
        "35_splice_strings" => Some(
            "Write a helper function `strlen` that takes a string pointer, iterates until it finds 0, and returns the length.\n\
             For s1, copy the first (len/2) bytes to s3.\n\
             For s2, start copying from the end of s2 (address + len - 1) backwards, for (len - len/2) bytes.\n\
             Don't forget to push parameters correctly and use `mov bp, sp` to access them inside `CopiaStringhe`."
        ),
        _ => None,
    }
}
