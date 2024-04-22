## Howto run

`cargo run mandel.png 1000x750 -1.20,0.35 -1,0.20`

This solution gets an almost perfect linear speedup from its threads. It splits the plotting area up into many more bands, and then has threads draw bands from a common pool until the pool is empty. When a thread finishes one band, it goes back for more work. Since the bands still take different amounts of time to render, the problem cited above still occurs, but on a much smaller scale.