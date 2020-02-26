# glep

Precise line regexp matching in Rust

## How to use

`glep file pattern line [lines]`

file: filename or - for stdin
pattern: Rust regexp pattern
line: Line where the pattern should first be matched

`glep` will check for a regular expression at a specific line in a file. If any
of the three first arguments are not provided, an error will be returned.

## Examples

`glep tests/simple.json "value_0" 1 2 4` will make sure that "value\_0" is present
on line 1, 2 and 4.

`glep tests/simple.json "value_[0-9]" 1 2 4` will make sure that the string "value\_"
followed by a single digit is present on line 1. Then, it will make sure that
the matched string (in our case, value\_0) is also present on lines 2 and 4.

## How to build

`cargo build --release` builds the glep binary. To install `glep` globally, use
`cargo install --path <path>`
