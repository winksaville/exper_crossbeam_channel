# Experiment with crossbeam_channel using no_std library

This example was fabricated by ChatGPT for target `thumbv7em-none-eabihf`.

The questions were:
 1. "Can crossbeam_channel be used in a no_std embedded rust environment?" A: Yes it's possible
 2. "Are you sure, when I look at the official documentation I could not find anything suggesting no_std was supported?"
 A: starts with: "You are correct that the official documentation for crossbeam_channel
 does not mention support for no_std environments. However, crossbeam_channel
 is built on top of the crossbeam crate, which does support no_std environments."
 3. "I'm a doubting Thomas, can you provide a simple example that would compile with rustc 1.66?" A: Proides something that "looks" real
 4. "Have you actually compiled the code?" A: No I can't

## Code from 3:

Short answer it doesn't work, at least the way I tried it :)

```
#![no_std]

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
    for message in receiver {
        println!("Received message: {}", message);
    }

    // Wait for the other thread to finish.
    thread.join().unwrap();
}
```

Trying to compile yields, so unsurprisingly the initial stab by ChatGPT
doesn't work:
```
wink@3900x 22-12-31T22:10:06.604Z:~/prgs/rust/myrepos/exper_crossbeam_channel (add-no_std)
$ cargo build --profile release-no_std-thumb7 --target thumbv7em-none-eabihf
   Compiling crossbeam-utils v0.8.14
error[E0463]: can't find crate for `std`
  |
  = note: the `thumbv7em-none-eabihf` target may not support the standard library
  = note: `std` is required by `crossbeam_utils` because it does not declare `#![no_std]`

error[E0463]: can't find crate for `std`
  --> /home/wink/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/lib.rs:83:24
   |
83 |         pub(crate) use std::sync::{Arc, Condvar, Mutex};
   |                        ^^^ can't find crate
   |
   = note: the `thumbv7em-none-eabihf` target may not support the standard library

...

error[E0405]: cannot find trait `Sync` in this scope
   --> /home/wink/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.14/src/thread.rs:471:16
    |
471 | unsafe impl<T> Sync for ScopedJoinHandle<'_, T> {}
    |                ^^^^ not found in this scope
    |
help: consider importing this trait
    |
114 | use core::marker::Sync;
    |

Some errors have detailed explanations: E0405, E0412, E0425, E0432, E0433, E0463, E0531.
For more information about an error, try `rustc --explain E0405`.
error: could not compile `crossbeam-utils` due to 232 previous errors
```

## License

See [COPYRIGHT.md](COPYRIGHT.md)
