## Howto run

`cargo run mandel.png 1000x750 -1.20,0.35 -1,0.20`

This solution uses the Rayon library. Rayon provides a parallel iterator API that makes our code much simpler. It looks a lot like Rust code that uses plain old iterators.