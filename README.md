My Project Euler solutions
==========================

###WARNING:
Project Euler asks not to publish solutions outside of projecteuler.net, but
I prefer to save my work in a git repository to reference it and share.
Proceed at your own risk. I don't claim my solutions are the most elegant
or efficient, only that they are my own.

###Currently implemented:
- 8:
  - go
- 10:
  - https://blog.golang.org/profiling-go-programs
  - go
  - Discovered that most time was spent in memmove
  - removed slice operation from program and it found primes under 2,000,000 in:
```bash
$ time ./primes
Result: 142913828922

real    4m38.983s
user    4m37.685s
sys     0m1.539s
```
- 11