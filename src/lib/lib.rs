use std::thread::{spawn, JoinHandle};

enum Status {
    Busy,
    Free,
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
    status: Status,
}

impl Worker {
    fn new(id: usize) -> Self {
        let thread = spawn(|| {});
        Worker {
            id,
            thread,
            status: Status::Free,
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    pub fn run<F>(&self, f: F)
    where
        F: FnOnce() -> (),
    {
    }
}
