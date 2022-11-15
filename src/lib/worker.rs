use std::{thread, sync::{mpsc::Receiver, Arc, Mutex}};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job()
                },
                Message::Terminate => {
                    println!("Worker {} got a stop request.", id);
                    break;
                }
            }

   
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }

    pub fn stop(&mut self) {
        println!("Stopping the worker {}", self.id);

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}