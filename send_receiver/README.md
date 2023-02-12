# Experiment with sending a receiver

Verify it's possible to send a Receiver to another thread via a crossbeam_channel.
Not only does it work, but nicely you can do it via a `BoxMsgAny` type!

## Run:

```
wink@3900x 23-02-12T15:58:03.651Z:~/prgs/rust/myrepos/exper_crossbeam_channel/send_receiver (main)
$ cargo run
   Compiling send_receiver v0.1.0 (/home/wink/prgs/rust/myrepos/exper_crossbeam_channel/send_receiver)
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `/home/wink/prgs/rust/myrepos/exper_crossbeam_channel/target/debug/send_receiver`
thread:+
thread: GOT a msg_receiver=MsgReceiver { receiver: Receiver { .. } }
thread: from msg_receiver.recv() GOT a msg_done=MsgDone
thread:-
```

## License

See [COPYRIGHT.md](COPYRIGHT.md)
