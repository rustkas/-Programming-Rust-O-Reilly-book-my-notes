## Howto run

`cargo run mandel.png 1000x750 -1.20,0.35 -1,0.20`

This solution uses Rust's atomic types to implement a lock-free iterator type, and uses that to dole out bands from the pool instead of a mutex-protected count. On Linux, this is no faster than the mutex-based version, which isn't too surprising: on Linux, locking and unlocking an uncontended mutex is simply a pair of atomic operations.