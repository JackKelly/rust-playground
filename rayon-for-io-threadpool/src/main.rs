use rayon::iter::{IntoParallelIterator, ParallelBridge};
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
        in_rx.into_iter().par_bridge().for_each_init(
            || {
                println!("outer init");
                out_tx.clone()
            },
            |out_tx, elem| {
                if elem < 5 {
                    (0..4).into_par_iter().for_each_init(
                        || {
                            println!("inner init");
                            out_tx.clone()
                        },
                        |out_tx, i| out_tx.send(i + elem + 100).unwrap(),
                    );
                }
                out_tx.send(elem * 10).unwrap()
            },
        );
    });

    for i in 0..32 {
        in_tx.send(i).unwrap();
    }

    // Finish up:
    drop(in_tx);

    for result in out_rx {
        println!("{result}");
    }

    println!("done");
}

