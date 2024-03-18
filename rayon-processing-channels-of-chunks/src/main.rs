use rayon::prelude::*;
use std::sync::mpsc::{channel, Receiver};

fn main() {
    let (in_tx, in_rx) = channel();
    let (out_tx, out_rx) = channel();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    pool.spawn(move || {
        in_rx
            .into_iter()
            .par_bridge()
            .for_each(|inner_rx: Receiver<i32>| {
                let out = inner_rx
                    .into_iter()
                    .par_bridge()
                    .map(|i| i + 1)
                    .reduce(|| 0, |a, b| a + b);
                out_tx.send(out).unwrap();
            });
    });

    // Submit 'tasks' to `in_tx`.
    for _ in 0..4 {
        let (inner_tx, inner_rx) = channel();
        for i in 10..14 {
            inner_tx.send(i).unwrap();
        }
        in_tx.send(inner_rx).unwrap();
    }

    // Finish up:
    drop(in_tx);

    for result in out_rx {
        println!("{result:?}");
    }

    println!("done");
}
