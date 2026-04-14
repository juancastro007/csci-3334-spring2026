// Assignment #3
/*
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// Message sent to workers
enum Message {
    NewJob(Job),
    Terminate,
}

// A job is a boxed closure
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Create a channel
        let (sender, receiver) = mpsc::channel();

        // Wrap receiver so multiple workers can share it safely
        let receiver = Arc::new(Mutex::new(receiver));

        // Create workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Return the thread pool
        ThreadPool { workers, sender }
    }

    // Execute a job in the pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Cleanup when pool goes out of scope
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers...");

        // Send one terminate message per worker
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers...");

        // Join all worker threads
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Worker struct
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // Lock receiver and wait for a message
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    // Create thread pool with 4 workers
    let pool = ThreadPool::new(4);

    // Submit 10 tasks
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }

    println!("Main thread waiting for tasks to complete...");
    // pool is dropped here, which sends terminate messages and joins workers
}*/

/////////////////////////////////////////////////////

// Assignment #4
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Special value to tell consumers to stop
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Total number of items to produce
    const ITEM_COUNT: usize = 20;

    // Number of producers and consumers
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Wrap receiver so multiple consumers can share it safely
    let rx = Arc::new(Mutex::new(rx));

    // Store thread handles
    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];

    // Split the work between 2 producers
    let items_per_producer = ITEM_COUNT / NUM_PRODUCERS;

    // Create 2 producer threads
    for id in 1..=NUM_PRODUCERS {
        let tx_clone = tx.clone();

        let handle = thread::spawn(move || {
            producer(id, tx_clone, items_per_producer);
        });

        producer_handles.push(handle);
    }

    // Create 3 consumer threads
    for id in 1..=NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);

        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });

        consumer_handles.push(handle);
    }

    // Wait for all producers to finish first
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Main thread sends one termination signal per consumer
    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for all consumers to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Producer sends random numbers into the channel
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for item_num in 1..=item_count {
        let value = rng.gen_range(1..=100);

        println!("Producer {} generated item {}: {}", id, item_num, value);

        tx.send(value).unwrap();

        // Simulate production time
        thread::sleep(Duration::from_millis(100));
    }

    println!("Producer {} finished producing.", id);
}

// Consumer receives numbers and processes them
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        // Receive one value from the channel
        let value = {
            let receiver = rx.lock().unwrap();
            receiver.recv().unwrap()
        };

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal and is exiting.", id);
            break;
        }

        println!("Consumer {} processed value {}", id, value);

        // Simulate work
        thread::sleep(Duration::from_millis(150));
    }
}