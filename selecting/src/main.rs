use std::{env::args, thread, time::Duration};

use crossbeam_channel::{bounded, select, unbounded, Select};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        println!("Usage {} {{ ready | select }} delay_ms", args[0]);
        return;
    }

    let (int_sender, int_receiver) = unbounded::<i32>();
    let (string_sender, string_receiver) = unbounded::<String>();
    let (done_sender, done_receiver) = bounded::<()>(1);

    let delay_ms: u64 = match args[2].as_str().parse() {
        Ok(v) => v,
        Err(why) => {
            println!("Unable to parse delay_ms to an u64, {why}");
            return;
        }
    };

    // Spawn a new thread to send messages to the channels
    let thread_handle = thread::spawn(move || {
        println!("thread:+");
        thread::sleep(Duration::from_millis(delay_ms));

        println!("thread:  send 1");
        _ = int_sender.send(1);

        thread::sleep(Duration::from_millis(delay_ms));

        println!("thread:  send hello");
        _ = string_sender.send("hello".to_string());

        thread::sleep(Duration::from_millis(delay_ms));

        println!("thread:  send world");
        _ = string_sender.send("world".to_string());

        thread::sleep(Duration::from_millis(delay_ms));

        println!("thread:  send 2");
        _ = int_sender.send(2);

        thread::sleep(Duration::from_millis(delay_ms));

        println!("thread:  send done ()");
        _ = done_sender.send(());

        thread::sleep(Duration::from_millis(delay_ms));
        println!("thread:-");
    });

    // Create a Select instance
    let mut sel = Select::new();

    // Add the receivers to the Select instance
    let idx = sel.recv(&int_receiver);
    assert_eq!(idx, 0);
    let idx = sel.recv(&string_receiver);
    assert_eq!(idx, 1);
    let idx = sel.recv(&done_receiver);
    assert_eq!(idx, 2);

    #[derive(Debug, PartialEq, Eq)]
    enum HandlerResult {
        Done,
        Continue,
    }

    // Create closures that capture the appropriate receiver
    // Turns out this code can't be shared between ready & select functions
    // because with select you have to use `oper.recv(&xxx)`.
    let handle_int_receiver: Box<dyn Fn() -> HandlerResult> = Box::new(|| {
        match int_receiver.recv() {
            Ok(message) => println!("Received an integer: {}", message),
            Err(why) => println!("Error int_receiver: {why:?}"),
        }

        HandlerResult::Continue
    });

    let handle_string_receiver: Box<dyn Fn() -> HandlerResult> = Box::new(|| {
        match string_receiver.recv() {
            Ok(message) => println!("Received a string: {}", message),
            Err(why) => println!("Error string_receiver: {why:?}"),
        }

        HandlerResult::Continue
    });

    let handle_done_receiver: Box<dyn Fn() -> HandlerResult> = Box::new(|| {
        match done_receiver.recv() {
            Ok(()) => println!("Done received"),
            Err(why) => println!("Error done_receiver: {why:?}"),
        }

        HandlerResult::Done
    });

    // Create array of handlers, this could be a `Vec<Box<dyn Fn() -> HandlerResult>>`
    // if a dynamic array was needed.
    let handlers = [
        handle_int_receiver,
        handle_string_receiver,
        handle_done_receiver,
    ];

    // Try different flavors of "selecting"; `sel.ready()`, `sel.select()`
    // and macro select!. I also learned how to create an array of closures!
    //
    // In this instance, where we have a fixed set of handlers for each of the
    // message types using `select!` is cleanest and easiest.
    match args[1].as_str() {
        "ready" => loop {
            println!("Top Ready Loop");

            // Wait for a message to be received on any of the added receivers
            let oper_index = sel.ready();

            // Run handler, this will panic if oper_index is to large
            let result = handlers[oper_index]();
            if result == HandlerResult::Done {
                break;
            };
        },
        "select" => 'select_loop: loop {
            println!("Top select Loop");

            // Wait for a message to be received on any of the added receivers
            let oper = sel.select();
            let oper_index = oper.index();
            match oper_index {
                0 => {
                    println!("select: int_receiver");
                    match oper.recv(&int_receiver) {
                        Ok(message) => println!("Received an integer: {}", message),
                        Err(why) => println!("Error int_receiver: {why:?}"),
                    }
                }
                1 => {
                    println!("select: string_receiver");
                    match oper.recv(&string_receiver) {
                        Ok(message) => println!("Received a string: {}", message),
                        Err(why) => println!("Error string_receiver: {why:?}"),
                    }
                }
                2 => {
                    println!("select: done_receiver");
                    match oper.recv(&done_receiver) {
                        Ok(()) => println!("Done received"),
                        Err(why) => println!("Error done_receiver: {why:?}"),
                    }

                    println!("select: done_receiver, break");
                    break 'select_loop;
                }
                _ => {
                    println!("Uknown oper_index={oper_index} select, ignoring");
                }
            }
        },
        "select!" => loop {
            println!("Top select! Loop");

            select! {
                recv(int_receiver) -> msg => match msg {
                    Ok(message) => println!("Received an integer: {}", message),
                    Err(why) => println!("Error int_receiver: {why:?}"),
                },
                recv(string_receiver) -> msg => match msg {
                    Ok(message) => println!("Received a string: {}", message),
                    Err(why) => println!("Error string_receiver: {why:?}"),
                },
                recv(done_receiver) -> msg => {
                    match msg {
                        Ok(()) => println!("Done received"),
                        Err(why) => println!("Error done_receiver: {why:?}"),
                    }

                    break;
                },
            }
        },
        _ => {
            println!(
                "First parameter was {}, expected \"ready\" or \"select\" or \"select!\"",
                args[1].as_str()
            );

            return;
        }
    }

    thread_handle.join().unwrap();
}
