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
}