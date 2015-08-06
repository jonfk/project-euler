My Project Euler solutions
==========================

###WARNING:
Project Euler asks not to publish solutions outside of projecteuler.net, but
I prefer to save my work in a git repository to reference it and share.
Proceed at your own risk. I don't claim my solutions are the most elegant
or efficient, only that they are my own.

###Currently implemented:
- 1:
  - Racket
  - Javascript
  - lua
  - nim
  - scala
  - haskell
- 2:
  - Racket
  - Haskell
- 3:
  - Haskell
  - Scala
    - using rosetta code bird implementation
- 4:
  - Haskell
- 5:
  - Haskell
- 6:
  - Haskell
- 7:
  - Haskell
- 8:
  - Go
  - Racket
  - Haskell
- 9:
  - Haskell
- 10:
  - go
    - https://blog.golang.org/profiling-go-programs
    - profiling no longer needed. time spent in memory allocation. Turn to static allocation with appending.
  - Haskell (incomplete)
- 11

###Languages To try
- Nim
- Javascript
- Lua
- Rust

###Notes

####Rust
Don't forget to compile with optimization. By default rustc compiles with no optimizations.
```bash
$ rustc -O 1.rs
# OR also
$ rustc -C opt-level=3
```