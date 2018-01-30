# Anagrams

A simple anagram finder program.


## What I've learned

- Pattern matching
- Basic file reading
- Basic ownership and lifetimes
- Basic modules (main executable + supporting modules)
- `rustup` usage (much prefer it over system's `rust` now)
- Basic `cargo` usage
- To use [clippy](https://github.com/rust-lang-nursery/rust-clippy) for a ton of useful warnings

Also see `LEARNING:` comments in the source code.

## TODO

- Tests
- Python equivalent

## How to run

Tested with Rust version 1.23.0.

```
$ cargo run --release -- --help
```

The simplest way to get this to work is to get a "words" file for your distribution.

See: <https://en.wikipedia.org/wiki/Words_(Unix)>

Example run:

```
$ cargo run --release -- -d /usr/share/dict/words loop pole
   Compiling anagrams v0.1.0 (file:///...)
    Finished release [optimized] target(s) in 4.88 secs
     Running `target/release/anagrams -w /usr/share/dict/words loop pole`
loop: Polo, polo, pool
pole: Opel, lope
```

## Achieving maximum performance

Release builds are LTO-enabled for performance.

Cargo enables easy configuration of local build flags per user.

If you want to compile for your native CPU architecture, add this to `~/.cargo/config`:

```
[build]
rustflags = "-C target-cpu=native"
```

Alternatively, you can compile the program with custom flags:

```
$ cargo rustc --release -- -C target-cpu=native
```

## Benchmarking

One can benchmark the program end-to-end using the excellent [hyperfine crate](https://github.com/sharkdp/hyperfine)

```
$ cargo install hyperfine	# assumes you have cargo .bin in your $PATH
$ hyperfine "target/release/anagrams act bear cat lope zebra" -w 5 -m 50
```
