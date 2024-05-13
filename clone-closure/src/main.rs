use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

fn run_closure_on_multiple_threads<F>(f: F) -> Vec<JoinHandle<()>>
where
    F: Fn() + Clone + Send + 'static,
{
    const N_THREADS: usize = 4;
    (0..N_THREADS)
        .map(|_| {
            let f_clone = f.clone();
            thread::spawn(move || (f_clone)())
        })
        .collect()
}

fn main() {
    let captured_string = String::from("hello");
    let (tx, rx) = mpsc::channel();

    // Spawn threads:
    let handles = run_closure_on_multiple_threads(move || {
        tx.send(format!(
            "{} from {:?}!",
            captured_string,
            thread::current().id()
        ))
        .unwrap()
    });

    // Join threads:
    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    // Print contents of the channel:
    rx.into_iter().for_each(|s| println!("{s}"));
}
