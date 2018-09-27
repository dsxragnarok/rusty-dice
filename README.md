# Rusty Dice
An implementation of **Dice Dude** in **Rust**.

**v1.0.0**

## Usage
```bash
rustydice ndx[+|-]m
```

To run in **interactive mode**, pass the `-i` or `--interactive` option.
You may pass an optional path to a file for the log output.
```bash
rustydice --interactive [/path/to/logfile]
```

- **n**: the number of times to roll the die
- **dx**: the die type
- **+m** | **-m**: the modifier

## Die Types
- **d2**: 2-sided die (coin toss)
- **d4**: 4-sided die
- **d6**: 6-sided die
- **d8**: 8-sided die
- **d10**: 10-sided die
- **d12**: 12-sided die
- **d20**: 20-sided die
- **d100**: 100-sided die

## Examples
```bash
> rustydice 1d6
You rolled 1d6 for 4
 >>> [4]
> rustydice 3d4-2
You rolled 3d4-2 for 4
 >>> [2, 3, 1]
> rustydice 4d8+5
You rolled 4d8+5 for 18
 >>> [2, 2, 6, 3]
```
