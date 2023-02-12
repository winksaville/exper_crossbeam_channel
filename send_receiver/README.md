# Experiment with sending a receiver

Verify it's possible to send a Receiver to another thread via a crossbeam_channel.
Not only does it work, but you can do it via a `BoxMsgAny` type!

## Run:

```
wink@3900x 23-02-12T19:01:58.779Z:~/prgs/rust/myrepos/exper_crossbeam_channel/send_receiver (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/exper_crossbeam_channel/target/release/send_receiver`
main:+
main: sending MsgReceiver { name: "rx1", receiver: Receiver { .. } } on wait_for_msg_receiver_tx
thread1:+
thread1: waiting for receiver, will panic if error receiving
thread1: GOT initial receiver=MsgReceiver { name: "rx1", receiver: Receiver { .. } }
thread1: waiting on rx1
main: sending MsgHello { greeting: "Hello, via tx1" } on tx1
main: sending () on tx1
thread1: GOT on rx1 MsgHello { greeting: "Hello, via tx1" }
thread1: waiting on rx1
thread1: GOT on rx1 ()
thread1: waiting on rx1
main: sending 1 on tx1
main: sending Receiver { .. } on tx1
thread1: GOT on rx1 1
thread1: waiting on rx1
thread1: GOT on rx1 Receiver { .. }
thread1: waiting on rx1
main: sending MsgReceiver { name: "rx2", receiver: Receiver { .. } } on tx1
main: sending MsgHello { greeting: "Hello, via tx2" } on tx2
thread1: GOT on rx1 MsgReceiver { name: "rx2", receiver: Receiver { .. } }, switching receivers
thread1: waiting on rx2
main: sending MsgDone on tx2, thread should stop
thread1: GOT on rx2 MsgHello { greeting: "Hello, via tx2" }
thread1: waiting on rx2
thread1: GOT on rx2 MsgDone, STOPPING thread
thread1:-
main:+
```

## License

See [COPYRIGHT.md](COPYRIGHT.md)
