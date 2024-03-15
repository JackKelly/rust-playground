use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;
use std::sync::mpsc::channel;

fn main() {
    let (in_tx, in_rx) = channel();
    let (out_tx, out_rx) = channel();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    pool.spawn(move || {
        in_rx.into_iter().for_each(|vec: Vec<i32>| {
            let out = vec.par_iter().map(|i| i + 1).reduce(|| 0, |a, b| a + b);
            out_tx.send(out).unwrap();
        });
    });

    for _ in 0..4 {
        let vec: Vec<i32> = (0..4).collect();
        in_tx.send(vec).unwrap();
    }

    // Finish up:
    drop(in_tx);

    for result in out_rx {
        println!("{result:?}");
    }

    println!("done");
}
