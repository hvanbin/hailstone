# Hailstone

A command-line tool for generating hailstone sequences, also known as the Collatz conjecture.

## What is a Hailstone Sequence?

The hailstone sequence follows these simple rules:
- If the number is even: divide by 2
- If the number is odd: multiply by 3 and add 1

The sequence continues until it reaches 1, and the conjecture states that all positive integers will eventually reach 1.

## Usage

```bash
# Generate one step from starting value 1 (default)
hailstone

# Start from 3 and generate 5 steps
hailstone -s 3 -n 5

# Enable verbose mode to see visual representation
hailstone -s 3 -n 5 -v

# Run in continuous loop mode
hailstone -s 7 -n 10 -l
```

## Options

- `-s, --start <NUMBER>`: Starting value for the sequence (default: 1)
- `-n, --number <COUNT>`: Number of iterations to perform (default: 1)
- `-v, --verbose`: Enable verbose output with visual representation using asterisks
- `-l, --loop`: Run in continuous loop mode
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## Examples

Basic usage:
```bash
$ hailstone -s 3 -n 7
counter: 7
10
05
16
08
04
02
01
```

With verbose output:
```bash
$ hailstone -s 3 -n 5 -v
counter: 5
10	**********
05	*****
16	****************
08	********
04	****
```

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
