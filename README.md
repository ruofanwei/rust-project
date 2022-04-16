## Learn Rust

### [Install Rust](https://www.rust-lang.org/tools/install)

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
$ source $HOME/.cargo/env
```

### Updating a Crate to Get a New Version

```bash
$ rustup update
```
```bash
$ cargo update
```
### compile program and run

after adding the rand crate as a dependency
```bash
$ cargo build
```

```bash
$ cargo run
```