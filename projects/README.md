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

### 2. Guessing game
- See the `main.rs` code to learn more in details
- `cargo build` will automatically download all dependencies from `Cargo.toml`
- `cargo update` to update any dependencies. By default, the dependencies in `Cargo.toml`, e.g. `rand = "0.8.5"` == `rand = "^0.8.5"`, which means any updates from 0.8.5 to < 0.9.0 will be automatically updated.
