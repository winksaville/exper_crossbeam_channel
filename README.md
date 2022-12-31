# Experiment with crossbeam_channel

At the moment just one sub-package, `with_std`, I plan on
creating a second sub-package `no_std` but that will be a
future commit.

## sub packages

 * [with_std](with_std/)

## Contributing

Pull-Requests are very welcom, but before commiting run `cargo xt pre-commit`
from root. This runs `cargo fmt`, `cargo clippy` and `cargo test` so formatting,
coding style remain consistent as practical and tests are passing.
```
wink@3900x 22-12-31T20:03:51.732Z:~/prgs/rust/myrepos/exper_crossbeam_channel (main)
$ cargo xt pre-commit
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/xtask pre-commit`
Run cargo fmt []
Run cargo clippy []
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
Run cargo test []
    Finished test [unoptimized + debuginfo] target(s) in 0.18s
     Running unittests src/main.rs (target/debug/deps/with_std_crossbeam_channel-5c73dae099083c1c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/xtask-1936f974a1470f62)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
