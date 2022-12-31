# Experiment with crossbeam_channel

Used 3 different techniques for "selecting" multiple channels and
in particular when they have different Types!

## Run:

```
wink@3900x 22-12-31T03:39:52.519Z:~/prgs/rust/myrepos/exper_crossbeam
$ cargo run ready 500
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/exper_crossbeam ready 500`
Top Ready Loop
thread:+
thread:  send 1
Received an integer: 1
Top Ready Loop
thread:  send hello
Received a string: hello
Top Ready Loop
thread:  send world
Received a string: world
Top Ready Loop
thread:  send 2
Received an integer: 2
Top Ready Loop
thread:  send done ()
Done received
thread:-
wink@3900x 22-12-31T03:40:10.006Z:~/prgs/rust/myrepos/exper_crossbeam
$ cargo run select 500
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/exper_crossbeam select 500`
Top select Loop
thread:+
thread:  send 1
select: int_receiver
Received an integer: 1
Top select Loop
thread:  send hello
select: string_receiver
Received a string: hello
Top select Loop
thread:  send world
select: string_receiver
Received a string: world
Top select Loop
thread:  send 2
select: int_receiver
Received an integer: 2
Top select Loop
thread:  send done ()
select: done_receiver
Done received
select: done_receiver, break
thread:-
wink@3900x 22-12-31T03:40:18.383Z:~/prgs/rust/myrepos/exper_crossbeam
$ cargo run select! 500
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/exper_crossbeam 'select'\!'' 500`
Top select! Loop
thread:+
thread:  send 1
Received an integer: 1
Top select! Loop
thread:  send hello
Received a string: hello
Top select! Loop
thread:  send world
Received a string: world
Top select! Loop
thread:  send 2
Received an integer: 2
Top select! Loop
thread:  send done ()
Done received
thread:-
wink@3900x 22-12-31T03:40:25.992Z:~/prgs/rust/myrepos/exper_crossbeam
```

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
