use std::{
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

struct ThreadPool {
    keep_running: Arc<AtomicBool>,
    thread_handles: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    fn new(n_threads: usize) -> Self {
        let keep_running = Arc::new(AtomicBool::new(true));
        let mut thread_handles = Vec::with_capacity(n_threads);
        for _ in 0..n_threads {
            let keep_running_thread = Arc::clone(&keep_running);
            let handle = thread::spawn(move || {
                while keep_running_thread.load(Relaxed) {
                    println!("Hello from thread {:?}", thread::current().id());
                    thread::sleep(Duration::from_millis(50));
                }
            });
            thread_handles.push(handle);
        }

        Self {
            keep_running,
            thread_handles,
        }
    }

    fn stop(&mut self) {
        self.keep_running.store(false, Relaxed);
        for handle in self.thread_handles.drain(..) {
            handle.join().unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.stop();
    }
}

fn main() {
    {
        let _threadpool = ThreadPool::new(8);
        thread::sleep(Duration::from_millis(100));
    }
    println!("Done!");
}
