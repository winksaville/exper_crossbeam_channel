# Experiment with crossbeam_channel using the std library

Used 3 different techniques for "selecting" multiple channels and
in particular when they have different Types!

## Run:

```
wink@3900x 22-12-31T20:00:25.924Z:~/prgs/rust/myrepos/exper_crossbeam_channel/with_std (main)
$ cargo run ready 10
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/exper_crossbeam_channel/target/debug/with_std_crossbeam_channel ready 10`
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
wink@3900x 22-12-31T20:00:44.369Z:~/prgs/rust/myrepos/exper_crossbeam_channel/with_std (main)
$ cargo run select 100
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/exper_crossbeam_channel/target/debug/with_std_crossbeam_channel select 100`
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
wink@3900x 22-12-31T20:00:52.159Z:~/prgs/rust/myrepos/exper_crossbeam_channel/with_std (main)
$ cargo run select! 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/exper_crossbeam_channel/target/debug/with_std_crossbeam_channel 'select'\!'' 1`
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
wink@3900x 22-12-31T20:00:58.020Z:~/prgs/rust/myrepos/exper_crossbeam_channel/with_std (main)
```

## License

See [COPYRIGHT.md](COPYRIGHT.md)
