use std::io;
use copy::copy_main;

fn main() {
    use std::io::Write;
    if let Err(err) = copy_main() {
        writeln!(io::stderr(), "error: {}", err).unwrap();
    }
}
