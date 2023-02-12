use std::{any::Any, thread};

use crossbeam_channel::{unbounded, Receiver, bounded};

type BoxMsgAny = Box<dyn Any + Send>;

#[derive(Debug, Clone)]
struct MsgReceiver {
    pub name: String,
    pub receiver: Receiver<BoxMsgAny>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
struct MsgHello { pub greeting: String }

#[derive(Debug, Clone)]
struct MsgDone {}

fn main() {
    let (wait_for_receiver_tx, wait_for_receiver_rx) = bounded::<MsgReceiver>(0);
    let (tx1, rx1) = unbounded::<BoxMsgAny>();
    let (tx2, rx2) = unbounded::<BoxMsgAny>();

    let thread1_wait_for_rx = wait_for_receiver_rx.clone();

    // Spawn a new thread to send messages to the channels
    let thread1_handle = thread::spawn(move || {
        println!("thread1:+");

        println!("thread1: waiting for receiver, will panic if not received");
        let mut receiver = thread1_wait_for_rx.recv().unwrap();
        println!("thread1: GOT initial receiver={receiver:?}");

        loop {
            println!("thread1: waiting on {}", receiver.name);
            let msg = receiver.receiver.recv().unwrap();
            if let Some(msg_received) = msg.downcast_ref::<MsgHello>() {
                println!("thread1: GOT on {} {msg_received:?}", receiver.name);
            } else if let Some(msg_receiver) = msg.downcast_ref::<Receiver<BoxMsgAny>>() {
                println!("thread1: GOT on {} {msg_receiver:?}", receiver.name);
            } else if let Some(msg_received) = msg.downcast_ref::<i32>() {
                println!("thread1: GOT on {} {msg_received:?}", receiver.name);
            } else if let Some(msg_received) = msg.downcast_ref::<()>() {
                println!("thread1: GOT on {} {msg_received:?}", receiver.name);
            } else if let Some(msg_received) = msg.downcast_ref::<MsgDone>() {
                println!("thread1: GOT on {} {msg_received:?}, STOPPING thread", receiver.name);
                break;
            } else if let Some(msg_received) = msg.downcast_ref::<MsgReceiver>() {
                println!("thread1: GOT on {} {msg_received:?}, switching receivers", receiver.name);
                receiver = msg_received.clone();
            } else {
                panic!("thread1: GOT on {} unknown msg", receiver.name);
            }
        }
        println!("thread1:-");
    });

    // NOTE: All of these sends will maybe sent before the thread even starts!

    // Set initial receiver
    wait_for_receiver_tx.send(MsgReceiver { name: "rx1".to_string(), receiver: rx1.clone()}).unwrap();

    // Send some messages that are NOT MsgReceiver
    let msg = Box::new(MsgHello { greeting: "Hello, via tx1".to_string() });
    println!("main: sending {msg:?} on tx1");
    tx1.send(msg).unwrap();
    let msg = Box::new(());
    println!("main: sending {msg:?} on tx1");
    tx1.send(msg).unwrap();
    let msg = Box::new(1);
    println!("main: sending {msg:?} on tx1");
    tx1.send(msg).unwrap();
    let msg = Box::new(rx1.clone());
    println!("main: sending {msg:?} on tx1");
    tx1.send(msg).unwrap();

    // Send new MsgReceiver
    let msg = Box::new(MsgReceiver { name: "rx2".to_string(), receiver: rx2.clone() });
    println!("main: sending {msg:?} on tx1");
    tx1.send(msg).unwrap();

    let msg = Box::new(MsgHello { greeting: "Hello, via tx2".to_string() });
    println!("main: sending {msg:?} on tx2");
    tx2.send(msg).unwrap();
    let msg = Box::new(MsgDone {});
    println!("main: sending {msg:?} on tx2, thread should stop");
    tx2.send(msg).unwrap();

    thread1_handle.join().unwrap();
}
