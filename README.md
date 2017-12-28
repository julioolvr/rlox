# rlox

[![status](https://travis-ci.org/julioolvr/rlox.svg?branch=master)](https://travis-ci.org/julioolvr/rlox)

Interpreter for the Lox language from [craftinginterpreters.com](http://www.craftinginterpreters.com/)
written in Rust.

This roughly follows the Java implementation (at the moment of this writing the C implementation isn't
available). There are many bugs to be found and ways to improve the code, but it works fairly well!

## Usage

```
# Run Lox's REPL
cargo run

# Execute a Lox file
cargo run -- some_file.lox
```

## Samples

There are some code samples going around in the tests, but I wrote a couple of small lox scripts that
can be used to test the interpreter - they can be found in `samples/`, e.g.

```
cargo run -- samples/closures.lox
```

## Playground

rlox is available online thanks to the wonders of WebAssembly. Check it out at
[rlox-wasm.now.sh](https://rlox-wasm.now.sh/).
