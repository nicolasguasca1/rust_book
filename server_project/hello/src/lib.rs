use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
struct Job;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            // create some workers and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers , sender }
    }

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {receiver});
        Worker {id,thread}
    }
}


    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //     if size > 0 {
    //         Ok(ThreadPool::new(size))
    //     } else {
    //         Err(PoolCreationError::InvalidSize)
    //     }
    // }

pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {}
}