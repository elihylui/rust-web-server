use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

//job will hold the closure to be sent to workers
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            //shared ownership & mutability - need thread safe smart pointers
            workers.push(Worker::new(id, Arc::clone(&receiver)))
            
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f:F)
    where 
        F: FnOnce() + Send + 'static
        {
            //use channel to send info from one thread to another 
            //here, use channel to send closure to one of the workers for processing
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .unwrap()
                .recv() //receive a job from the channel
                .unwrap();

            println!("Worker{} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}
