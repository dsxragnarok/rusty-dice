# Dice Dude in Rust
## Dice module
### die types (enums)
**d2, d4, d6, d8, d10, d12, d20, d100**

### Roll struct
#### properties
- die
- number_of_rolls
- modifier

#### private methods
- _roll() -> u32

#### public methods
- new(die) -> Dice
- number_of_rolls(n: u32) -> ()
- modifier(m: i32) -> ()
- roll() -> RollResult

### RollResult struct
- die: Die
- rolls: Vec<u32>
- modifier: i32
- total: i32

## Command module
### commands
- dx
- ndx
- ndx[+|-]m
- ndx [+|-] m

## Logger module
### format
```bash
[09/14/2018 22:03:42] You rolled 2d6+3 for 13.
    >>> { 6, 4 }
```
