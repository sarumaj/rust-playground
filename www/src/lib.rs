use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
    time::Duration,
};

#[cfg(test)]
mod tests {
    use super::ThreadPool;
    use std::{thread, time::Duration};

    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(4);
        for i in 0..12 {
            pool.execute(move || {
                println!("Job {} is running.", i);
            })
            .unwrap();
        }

        thread::sleep(Duration::from_secs(1));
        drop(pool);
    }
}

/// A type alias for a result that only contains an error.
pub type ErrorOnly = Result<(), Box<dyn Error>>;

/// A type alias for a job that can be sent to a thread pool.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A worker that can execute jobs.
struct Worker {
    /// The ID of the worker.
    id: usize,
    /// The thread that the worker is running on.
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new worker.
    /// The worker will run in a loop, waiting for jobs to execute.
    /// When a job is received, the worker will execute it.
    /// The worker will stop running when the sender is dropped or
    /// when the should_terminate flag is set to true.
    /// To avoid busy waiting, the worker will sleep for 10 milliseconds.
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
        should_terminate: Arc<AtomicBool>,
    ) -> Worker {
        thread::Builder::new()
            .name(format!("Worker-{}", id))
            .spawn(move || loop {
                if should_terminate.load(Ordering::Acquire) {
                    return;
                }

                match receiver.lock().unwrap().recv() {
                    Ok(job) => job(),
                    Err(_) => return,
                }

                thread::sleep(Duration::from_millis(10));
            })
            .map(|thread| Worker {
                id,
                thread: Some(thread),
            })
            .expect("Failed to create a new worker thread.")
    }
}

/// A thread pool that can execute jobs.
pub struct ThreadPool {
    /// The workers in the thread pool.
    workers: Vec<Worker>,
    /// The sender that can be used to send jobs to the workers.
    sender: mpsc::Sender<Job>,
    /// Flag to signal workers to terminate immediately.
    should_terminate: Arc<AtomicBool>,
}

impl ThreadPool {
    /// Create a new thread pool with the given number of workers.
    /// The thread pool will start the specified number of workers.
    /// The workers will run in a loop, waiting for jobs to execute.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // Wrap the receiver in an Arc and a Mutex to share it among workers.
        let receiver = Arc::new(Mutex::new(receiver));
        // Wrap the should_terminate flag in an Arc to share it among workers.
        let should_terminate = Arc::new(AtomicBool::new(false));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                Arc::clone(&should_terminate),
            ));
        }

        ThreadPool {
            workers,
            sender,
            should_terminate,
        }
    }

    /// Execute a job in the thread pool.
    /// The job will be sent to one of the workers in the thread pool.
    /// If the thread pool and its sender is dropped, the job will not be executed.
    pub fn execute<F>(&self, f: F) -> ErrorOnly
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f))?;
        Ok(())
    }
}

impl Drop for ThreadPool {
    /// Drop the thread pool.
    /// The thread pool will signal the workers to terminate immediately.
    /// The thread pool will wait for all workers to finish before returning.
    fn drop(&mut self) {
        // Signal workers to terminate immediately.
        self.should_terminate.store(true, Ordering::Release);

        // Send an empty closures to each worker to wake them up.
        for _ in &self.workers {
            self.sender.send(Box::new(|| {})).unwrap();
        }

        // Wait for all workers to finish.
        for worker in &mut self.workers {
            // Take the thread handle from the worker and join it.
            // Join will take the ownership of the thread handle.
            // To avoid leaking the thread handle, we take it and leave None in its place.
            if let Some(thread) = worker.thread.take() {
                if let Err(e) = thread.join() {
                    eprintln!("Worker {} did not shut down properly: {:?}", worker.id, e);
                }
            }
        }
    }
}
