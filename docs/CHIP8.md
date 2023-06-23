# CHIP8

## Status

All of the games written for the original version of CHIP8 should work flawlessly.

## Information used

- [Awesome CHIP8](https://chip-8.github.io/links/)
- [Timendus' CHIP8 test suite](https://github.com/Timendus/chip8-test-suite)

## Problems

- The sound in terminal mode works only if the terminal emulator you are using supports the printing of the \x07 character as a beep. Most of them should support this.
- In terminal mode, key release events are registered only on special terminals that support the [kitty protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/).
    - This is a limitation of the crate crossterm, until it gets fixed there, there's nothing I can do about it.