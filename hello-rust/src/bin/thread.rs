/*!
 * Rust Threading Example
 * 
 * This program demonstrates basic multithreading in Rust using the standard library.
 * It creates a spawned thread that runs concurrently with the main thread,
 * showing how threads can execute independently and how to synchronize them.
 */

use std::{thread, time::Duration};

/**
 * Main function that demonstrates concurrent execution using threads.
 * 
 * The program creates two concurrent execution paths:
 * 1. A spawned thread that counts from 1 to 9
 * 2. The main thread that counts from 1 to 4
 * 
 * Both threads execute simultaneously, with small pauses between iterations
 * to make the concurrent execution visible in the output.
 */
fn main() {
    // Spawn a new thread that will run concurrently with the main thread
    // The closure (||) defines the code that will run in the spawned thread
    let handle = thread::spawn(|| {
        // Loop from 1 to 9 (exclusive upper bound)
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            
            // Sleep for 1 millisecond to:
            // 1. Allow other threads to execute (yield control)
            // 2. Make the interleaving of thread execution visible
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread execution - runs concurrently with the spawned thread
    // Loop from 1 to 4 (exclusive upper bound)
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        
        // Sleep for 1 millisecond for the same reasons as above
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to complete before the main thread exits
    // handle.join() blocks the main thread until the spawned thread finishes
    // .unwrap() panics if the spawned thread panicked (error handling)
    handle.join().unwrap();
    
    // Note: If we didn't call join(), the main thread might exit before
    // the spawned thread completes, potentially terminating the spawned
    // thread prematurely.
}

/*
 * Expected Output Pattern:
 * The output will be interleaved between both threads, something like:
 * 
 * hi number 1 from the main thread!
 * hi number 1 from the spawned thread!
 * hi number 2 from the main thread!
 * hi number 2 from the spawned thread!
 * hi number 3 from the main thread!
 * hi number 3 from the spawned thread!
 * hi number 4 from the main thread!
 * hi number 4 from the spawned thread!
 * hi number 5 from the spawned thread!
 * hi number 6 from the spawned thread!
 * hi number 7 from the spawned thread!
 * hi number 8 from the spawned thread!
 * hi number 9 from the spawned thread!
 * 
 * Note: The exact interleaving may vary between runs due to thread scheduling.
 */