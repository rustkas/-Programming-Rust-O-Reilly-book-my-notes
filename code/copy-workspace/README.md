## How to make library project:
`mkdir members && cd members && cargo new copy-lib --lib --vcs none`

`cargo new copy-main --bin --vcs none`

`cargo run --package copy-main --bin copy-main test.txt test-folder`

`cargo run --package copy-main --bin copy-main`