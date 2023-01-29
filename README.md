# rust-compiler

A compiler written in Rust to compile a made-up language. Basically my definition of a fun night.

## Running

You'll need [Rust](https://www.rust-lang.org/) and Cargo

Each step of the compiler is it's own binary under `./src/bin` (which each step building upon the previous step).

You can run each step individually with `cargo run --bin <bin name> -- <files>`, ie. `cargo run --bin lexdriver -- ./test`

A set of test files are included under `./test` to try things out.
