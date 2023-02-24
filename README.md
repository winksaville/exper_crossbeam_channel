# Experiment with crossbeam_channel

At the moment two sub-packages, `selecting` and `send_receiver`.

I also thought crossbeam_channel (cc) was capable of `no_std` but it
isn't. I thought it might be because ChatGPT suggested it might be.
Wrong, don't trust a bot, it gives reasonable looking answers but
user beware :)

Anyway, after failing I looked deeper and right there on the
[crossbeam README file](https://github.com/crossbeam-rs/crossbeam)
it says:

*Features marked with (no_std) can be used in no_std environments.*
Features marked with (alloc) can be used in no_std environments, but only if alloc feature is enabled

Turns out cc isn't one of them. In particular cc is dependent upon
thread synchronization tools and it isn't marked with (on_std).
There are probably others as there were 200+ errors while compiling
cc in my add-no_std branch of exper_crossbeam_channel. See more
details [here](https://github.com/winksaville/exper_crossbeam_channel/tree/add-no_std/no_std).

So I'll stop developement on no_std as it's not going to bare
any fruit in the near term.

## sub packages

* [selecting](selecting/):
  * Test three ways of selecting, `ready`, `select` and `select!`.
* [send_receiver](send_receiver/)
  * Verify it's possible to send a Receiver to another thread via a crossbeam_channel.
   Not only does it work, but you can do it via a `BoxMsgAny` type!

## Contributing

Pull-Requests are very welcome, but before commiting run `cargo xt pre-commit`
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
