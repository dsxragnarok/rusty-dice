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
- new(die: Die) -> Roll
- number_of_rolls(n: u32) -> ()
- modifier(m: i32) -> ()
- roll() -> RollResult

### RollResult struct
- die: Die
- rolls: Vec<u32>
- modifier: i32
- total: i32

## Command module
### Command
#### members
- `n`
- `die`
- `modifier`
#### methods
- new(n: u32, die: Die, modifier: i32) -> Command
- from(input_text: &str) -> Command

### syntax
- `dx`: roll a **x-sided** die **once**.
- `ndx`: roll a **x-sided** die **n times**.
- `ndx+m`: roll a **x-sided** die **n times** then add **m** to the result.
- `ndx-m`: roll a **x-sided** die **n times** then subtract **m** from the result.
### functions
- parse(input: String) -> Command

## Logger module
### functions
- build_log(result) -> String

### format
```bash
[09/14/2018 22:03:42] You rolled 2d6+3 for 13.
    >>> { 6, 4 }
```

## Notes
- [ ] `Command` needs a `run()` or `execute()`
- [x] refactor `command::parse()` to `Command::from()`
- [ ] implement the commandline interface module
- [ ] implement interactive mode
