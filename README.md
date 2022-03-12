# rust_microservice_boilerplate

### Create service
* `cargo new example_service`

### Run tests
* `cargo test`
```
❯ cargo test
   Compiling example_service v0.1.0 (/Users/sumit/Downloads/Code/rust_microservice_boilerplate/example_service)
    Finished test [unoptimized + debuginfo] target(s) in 0.75s
     Running unittests (target/debug/deps/example_service-9d8a62a6b6d0a920)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Run Cargo check
* `cargo check`

```
❯ cargo check
    Checking example_service v0.1.0 (/Users/sumit/Downloads/Code/rust_microservice_boilerplate/example_service)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
```

### Run the project
* `cargo run`
```
❯ cargo run
   Compiling example_service v0.1.0 (/Users/sumit/Downloads/Code/rust_microservice_boilerplate/example_service)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/example_service`
Hello, world!

```

### Run Linter
* `cargo clippy`
* `cargo clippy -- -D warnings`
### Run format
* `cargo fmt`
* `cargo fmt -- --check`

### Run code coverage
* `cargo install cargo-tarpaulin`
* `cargo tarpaulin --ignore-tests`

### Run audit
* `cargo install cargo-audit`
* `cargo audit`

### How to add new package?
* Add package name from crates.io to Cargo.toml and run `cargo check`