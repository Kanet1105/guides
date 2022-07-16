use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

use crate::parallel::exception::ThreadCountError;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> Result<Self, ThreadCountError> {
        if thread_count <= 0 {
            return Err(ThreadCountError)
        }

        let (sender, receiver) = mpsc::channel();
        let shared_receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::<Worker>::with_capacity(thread_count);
        for id in 0..thread_count {
            workers.push(Worker::new(id, Arc::clone(&shared_receiver)));
        }

        let threadpool = Self {
            workers,
            sender,
        };

        Ok(threadpool)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    inner: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {}", id);
            job();
        });

        Self {
            id,
            inner: thread,
        }
    }
}