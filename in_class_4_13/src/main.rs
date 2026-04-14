/*
// Assignment #1
use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread starting");
    
    // Vector to store thread handles
    let mut handles = vec![];
    
    // Spawn 3 threads
    for i in 1..=3 {
        let handle = thread::spawn(move || {
            // Simulate some work
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });
        
        // Store the handle so we can join later
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap(); // blocks until thread finishes
    }
    
    println!("All threads completed.");
}*/
/////////////////////////////////////////////////////

// Assignment #2
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter starting at 0
    let counter = Arc::new(Mutex::new(0));

    // Vector to store thread handles
    let mut handles = vec![];

    // Spawn 5 threads
    for i in 1..=5 {
        // Clone the Arc so each thread gets shared ownership
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Increment the counter 10 times
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }

            println!("Thread {} done", i);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}