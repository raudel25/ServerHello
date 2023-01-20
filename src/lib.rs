use std::thread;

pub struct ThreadPool {
    threads: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<Self, &'static str> {
        if size == 0 {
            Err::<Self, &'static str>("size must be greater than 0");
        }

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i));
        }

        Ok(ThreadPool { threads })
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

impl Worker {
    fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
