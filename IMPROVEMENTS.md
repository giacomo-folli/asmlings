# Improvements

## 1. User exercise completion flow

- **[high / med] Add `reset`, `skip`, and `back` commands.** Progress is a single index in `.asmlings_state` (`state.rs`). A stuck student can't skip forward, and can't revisit a solved exercise without hand-editing the state file. Add subcommands in `main.rs`/`commands.rs` that adjust the index (and, for `reset`, re-extract the current exercise from the embedded templates).
- **[med / med] Progressive, tiered hints.** `h` shows one flat hint, once per exercise (the `hint_shown` gate at `commands.rs:339`). Let repeated `h` presses reveal successively more detail (nudge → approach → near-solution) instead of gating after the first.
- **[med / low] Disambiguate emulation errors.** `emulator.rs:132` reports "Emulation failed or timed out (infinite loop?)" for every failure — a bad opcode, a memory fault, and a real infinite loop all look identical. Match on the Unicorn error to distinguish "hit the instruction cap (likely infinite loop)" from "invalid instruction / memory fault," so feedback points the student at the right problem.
- **[med / med] Harden trivial exercises against hardcoding.** A single-case suite (e.g. `exercise_tests.rs:70`) can't tell `mov ax, 0x1337` from an actual computation. The multi-`setup`-case pattern already used by `35_splice_strings` (`exercise_tests.rs:637`) randomizes/varies inputs so the answer can't be pasted — spread it to the arithmetic/logic exercises. See §2 for the mechanism.
- **[low / low] Debounce is a single timestamp.** `last_run` (`commands.rs:338`) can still double-fire on editors that emit several write events per save; a short trailing debounce (coalesce events within the window) is more robust. Minor.

## 2. Exercise testing

- **[med / med] Collapse trivial suites with a helper/macro.** ~26 of the 35 suites are a single hardcoded `check_reg` with no `setup` (e.g. `exercise_tests.rs:70-99`), which is why the file is 721 lines. A `simple_reg!(name, REG, expected)` helper (or a compact table) would turn each of those into one line while keeping the programmatic model. Reserve the full `ProgrammaticCase { setup, verify }` form for exercises that actually need setup or multiple cases.
- **[med / low] Wire up `check_flag` — it's dead code.** `harness.rs:32` carries `#[allow(dead_code)]` because no suite checks flags. Yet the flag-teaching exercises (`17_cmp`, `25_carry_flag`, `28_sign_comp`) verify a register side-effect instead of the flag the lesson is about. Have those suites assert the flag directly via `check_flag`.

## 3. General architecture

- **[med / low] Centralize emulator bootstrap.** `run_programmatic_suite` (`emulator.rs:41`) and `run_legacy_exercise` (`emulator.rs:115`) each repeat the same `Unicorn::new` → `mem_map(MEM_BASE, MEM_SIZE)` → `mem_write(LOAD_ADDR, code)` → `SP = 0xFFF0` setup. Factor it into one `fn new_emu(code) -> Unicorn`. (If the inline path is deleted per §2, this collapses naturally — but the setup is still worth isolating for readability and for the per-case loop.)
- **[low / low] Simplify the run flow once legacy is gone.** With the inline path removed, `Exercise` no longer needs `assertions`/`is_done`-from-directives parsing, and `run_workflow`'s `assertions.is_empty() && get_test_suite().is_none()` special-case (`commands.rs:131`) reduces to "is there a suite?". Fewer states to reason about.

## 4. Exercise suite

- **[med / low] Rename `30_test_1` / `31_test_2` / `32_test_3`.** Opaque names give the student no idea what's being taught. Rename to the skill (e.g. the popcount one at `exercise_tests.rs:579` → `32_popcount`). Renames must stay in sync across the `.asm` filename, the suite string in `exercise_tests.rs`, and the hint key in `hints.rs`.
- **[med / med] Fill progression gaps.** There is no exercise on conditional/unconditional jumps (`jmp`, `jz`, `jnz`) before `18_loop`, which uses looping — add one so control flow is introduced before it's assumed. Add flag-assertion exercises (pairs with the `check_flag` work in §2) since flags are core to branching yet never directly asserted. Consider mul/div edge cases (the `DX:AX` high word, division overflow) beyond the single happy path in `14_mul`/`15_div`.
- **[low / med] Lean on `target_label` subroutines for advanced exercises.** The subroutine-call harness (`emulator.rs:68`, sentinel return address) plus multi-case `setup` is the strongest anti-hardcoding tool in the codebase — new advanced exercises should be authored as callable routines tested against several inputs, like `35_splice_strings`.