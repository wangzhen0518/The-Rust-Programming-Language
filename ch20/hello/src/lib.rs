use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    ///     
    /// # Examples
    ///
    /// Create a thread pool with 4 threads:
    ///     
    /// ```
    /// use hello::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    /// Execute a function in a thread pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use hello::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    ///
    /// pool.execute(|| {
    ///    println!("Hello, world!");
    /// });
    /// ```
    ///
    /// # Panics
    ///
    /// The `execute` function will panic if the thread pool has been dropped.
    ///
    /// ```
    /// use hello::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    ///
    /// drop(pool);
    ///
    /// pool.execute(|| {
    ///   println!("Hello, world!");
    /// });
    /// ```
    ///
    /// The `execute` function will panic if the closure panics.
    ///
    /// ```
    /// use hello::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    ///
    /// pool.execute(|| {
    ///  panic!("oops!");
    /// });
    /// ```
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
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
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
