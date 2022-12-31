#![no_std]
/// Fabricated by ChatGPT

extern crate alloc;
extern crate core;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use crossbeam_channel::{bounded, select, Receiver, Sender};

fn main() {
    // Set up a channel with capacity 1.
    let (sender, receiver) = bounded(1);

    // Spawn a new thread that sends messages to the main thread.
    let thread_sender = sender.clone();
    let thread = core::thread::spawn(move || {
        for i in 0..10 {
            thread_sender.send(i).unwrap();
        }
    });

    // In the main thread, receive and print messages.
    let mut count = 0;
    for _message in receiver {
        //println!("Received message: {}", message);
        count += 1;
    }
    assert_eq!(count, 10);

    // Wait for the other thread to finish.
    thread.join().unwrap();
}
