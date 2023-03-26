# Learn Rust

## Introduction
### 1. Getting started
```bash
cargo --version

cargo new hello_world # creating new project
```

- `Cargo.toml` is where the libraries stored

```bash
cargo build # by default, it's creating debug version
cargo run # again, will run debug version by default
```

- `cargo build --release` will build the release version of the program
- Difference: dev faster to build but not optimised, release takes time to build but optimised
- `cargo run --release` will run the release version
- `cargo check` build a project without producing binary to check for some errors