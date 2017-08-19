# Anagrams

A simple anagram finder program.


## What I've learned

- Pattern matching
- Basic file reading
- Basic ownership
- Basic modules (main executable + supporting "library")

## TODO

- Tests
- Python equivalent
- Cleanup (separate into modules to learn about how Rust handles modularity)

## How to run

Tested with Rust version 1.19.0.

```
$ cargo run --release -- --help
```

The simplest way to get this to work is to get a "words" file for your distribution.

See: https://en.wikipedia.org/wiki/Words_(Unix)

Example run:
```
$ cargo run --release -- -w /usr/share/dict/words loop pole
   Compiling anagrams v0.1.0 (file:///...)
    Finished release [optimized] target(s) in 4.88 secs
     Running `target/release/anagrams -w /usr/share/dict/words loop pole`
loop: ["Polo", "polo", "pool"]
pole: ["Opel", "lope"]
```

