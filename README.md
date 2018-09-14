# Dice Dude in Rust
## Dice module
### die types (enums)
**d2, d4, d6, d8, d10, d12, d20, d100**

### properties
- sides
- n
- mod

### private methods
- _roll() -> u32

### public methods
- new(die) -> Dice
- n(n: u32) -> ()
- modifier(modifier: i32) -> ()
- roll() -> RollResult

## RollResult module
- die
- rolls
- mod
- total

### methods

## Command module
### commands
- dx
- ndx
- ndx[+|-]m
- ndx [+|-] m
