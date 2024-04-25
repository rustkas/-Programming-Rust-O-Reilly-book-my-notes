use echo_server::echo_main;

fn main() {
    echo_main("127.0.0.1:17007").expect("error: ");
}
