use rand::seq::SliceRandom;
use std::thread;
use std::time::Instant;

const MAX_NUMBER: usize = 100_000;

/// Really inefficient prime number calculator
fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2..n {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut candidates: Vec<usize> = (0..MAX_NUMBER).collect();
    // Perform the calculation
    let start = Instant::now(); // We're not timing the initial creation

    // Shuffle, so that the work is more evenly distributed between threads:
    candidates.shuffle(&mut rand::thread_rng());
    let mut primes: Vec<usize> = Vec::with_capacity(10_000);

    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(num_cpus::get());

        let chunks = candidates.chunks(candidates.len() / num_cpus::get());

        // Iterate each chunk
        for (id, chunk) in chunks.enumerate() {
            println!("Thread #{id} is using chunk size: {}", chunk.len());
            let handle = scope.spawn(move || {
                let chunk_start = Instant::now();

                let local_results: Vec<usize> =
                    chunk.iter().filter(|n| is_prime(**n)).map(|n| *n).collect();

                let chunk_elapsed = chunk_start.elapsed();
                println!(
                    "Thread #{id} took {:.4} seconds.",
                    chunk_elapsed.as_secs_f32()
                );
                local_results
            });

            handles.push(handle);
        }

        for handle in handles {
            let local_result: Vec<usize> = handle.join().unwrap();
            primes.extend(local_result);
        }
    });
    // Time how long it took
    let elapsed = start.elapsed();

    // Results
    println!(
        "Found {} primes, out of {} candidates.",
        primes.len(),
        candidates.len()
    );
    println!("Calculated in {:.4} seconds.", elapsed.as_secs_f32());
}
