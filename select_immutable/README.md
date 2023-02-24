# Experiment with creating SelectImmutable

I want to see if making a immutable select will allow
me to create receivers dynamically. Currently this is
just an example and doesn't try to create receivers
dynamically.

## Run:

```
wink@3900x 23-02-24T00:13:46.628Z:~/prgs/rust/myrepos/exper_crossbeam_channel/select_immutable (main)
$ cargo run ready 10
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/exper_crossbeam_channel/target/debug/select_immutable ready 10`
SelectImmutable::new:+=
SelectImmutable.recv:+
SelectImmutable.recv:- r=0
SelectImmutable.recv:+
SelectImmutable.recv:- r=1
SelectImmutable.recv:+
SelectImmutable.recv:- r=2
Top Ready Loop
SelectImmutable.ready:+
thread:+
thread:  send 1
SelectImmutable.ready:- r=0
Received an integer: 1
Top Ready Loop
SelectImmutable.ready:+
thread:  send hello
SelectImmutable.ready:- r=1
Received a string: hello
Top Ready Loop
SelectImmutable.ready:+
thread:  send world
SelectImmutable.ready:- r=1
Received a string: world
Top Ready Loop
SelectImmutable.ready:+
thread:  send 2
SelectImmutable.ready:- r=0
Received an integer: 2
Top Ready Loop
SelectImmutable.ready:+
thread:  send done ()
SelectImmutable.ready:- r=2
Done received
thread:-
wink@3900x 23-02-24T00:14:08.555Z:~/prgs/rust/myrepos/exper_crossbeam_channel/select_immutable (main)
$ cargo run select 100
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/exper_crossbeam_channel/target/debug/select_immutable select 100`
SelectImmutable::new:+=
SelectImmutable.recv:+
SelectImmutable.recv:- r=0
SelectImmutable.recv:+
SelectImmutable.recv:- r=1
SelectImmutable.recv:+
SelectImmutable.recv:- r=2
Top select Loop
SelectImmutable.slect:+
thread:+
thread:  send 1
SelectImmutable.select:- r=SelectedOperation { .. }
select: int_receiver
Received an integer: 1
Top select Loop
SelectImmutable.slect:+
thread:  send hello
SelectImmutable.select:- r=SelectedOperation { .. }
select: string_receiver
Received a string: hello
Top select Loop
SelectImmutable.slect:+
thread:  send world
SelectImmutable.select:- r=SelectedOperation { .. }
select: string_receiver
Received a string: world
Top select Loop
SelectImmutable.slect:+
thread:  send 2
SelectImmutable.select:- r=SelectedOperation { .. }
select: int_receiver
Received an integer: 2
Top select Loop
SelectImmutable.slect:+
thread:  send done ()
SelectImmutable.select:- r=SelectedOperation { .. }
select: done_receiver
Done received
select: done_receiver, break
thread:-
wink@3900x 23-02-24T00:14:16.712Z:~/prgs/rust/myrepos/exper_crossbeam_channel/select_immutable (main)
$ cargo run select! 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/exper_crossbeam_channel/target/debug/select_immutable 'select'\!'' 1`
SelectImmutable::new:+=
SelectImmutable.recv:+
SelectImmutable.recv:- r=0
SelectImmutable.recv:+
SelectImmutable.recv:- r=1
SelectImmutable.recv:+
SelectImmutable.recv:- r=2

No select! for select_immutable, so no responses to the `thread` messages
thread:+
thread:  send 1
thread:  send hello
thread:  send world
thread:  send 2
thread:  send done ()
thread:-
wink@3900x 23-02-24T00:14:25.820Z:~/prgs/rust/myrepos/exper_crossbeam_channel/select_immutable (main)```

## License

See [COPYRIGHT.md](COPYRIGHT.md)
