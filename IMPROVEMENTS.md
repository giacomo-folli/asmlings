# Improvements

## 1. User exercise completion flow

- **[med / med] Progressive, tiered hints.** `h` shows one flat hint, once per exercise (the `hint_shown` gate at `commands.rs:339`). Let repeated `h` presses reveal successively more detail (nudge → approach → near-solution) instead of gating after the first.
- **[med / med] Harden trivial exercises against hardcoding.** A single-case suite (e.g. `exercise_tests.rs:70`) can't tell `mov ax, 0x1337` from an actual computation. The multi-`setup`-case pattern already used by `35_splice_strings` (`exercise_tests.rs:637`) randomizes/varies inputs so the answer can't be pasted — spread it to the arithmetic/logic exercises. See §2 for the mechanism.
- **[low / low] Debounce is a single timestamp.** `last_run` (`commands.rs:338`) can still double-fire on editors that emit several write events per save; a short trailing debounce (coalesce events within the window) is more robust. Minor.

## 2. Other

- **Wire up `check_flag`** `harness.rs:32` carries `#[allow(dead_code)]` because no suite checks flags. Yet the flag-teaching exercises (`17_cmp`, `25_carry_flag`, `28_sign_comp`) verify a register side-effect instead of the flag the lesson is about. Have those suites assert the flag directly via `check_flag`.
- **Fill progression gaps.** Add flag-assertion exercises (pairs with the `check_flag` work in §2) since flags are core to branching yet never directly asserted. Consider mul/div edge cases (the `DX:AX` high word, division overflow) beyond the single happy path in `14_mul`/`15_div`.