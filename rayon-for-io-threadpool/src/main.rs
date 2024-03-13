use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::sync::mpsc::channel;

fn main() {
    let (in_tx, in_rx) = channel();
    let (out_tx, out_rx) = channel();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    let in_tx2 = in_tx.clone();
    pool.spawn(move || {
        in_rx.into_iter().par_bridge().for_each_init(
            || (out_tx.clone(), in_tx2.clone()),
            |txs, elem| {
                let (out_tx, in_tx) = txs;
                if elem == 50000000 {
                    in_tx.send(elem * 1000).unwrap();
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

