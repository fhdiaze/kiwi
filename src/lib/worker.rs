use std::{thread, sync::mpsc::Receiver};

use crate::job::Job;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Receiver<Job>) -> Self {
        let thread = thread::spawn(|| {
            receiver
        });
        Worker {
            id,
            thread,
        }
    }

    pub fn run<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static
    {
        self.thread.
    }
}