use rayon::iter::{IntoParallelIterator, ParallelBridge};
use rayon::prelude::ParallelIterator;
use std::sync::mpsc::channel;
use thread_local::ThreadLocal;

fn main() {
    let (in_tx, in_rx) = channel();
    let (out_tx, out_rx) = channel();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();
    let tls = ThreadLocal::new();

    pool.spawn(move || {
        in_rx.into_iter().par_bridge().for_each(|elem| {
            // Let's pretend that we've got some additional operations
            // to add to the task list:
            if elem < 5 {
                (0..4).into_par_iter().for_each(|i| {
                    let out_tx2 = tls.get_or(|| {
                        println!("Creating new out_tx clone (inner)!");
                        out_tx.clone()
                    });
                    out_tx2.send(i + elem + 100).unwrap()
                });
            }

            // We're ready to send a results back:
            let out_tx2 = tls.get_or(|| {
                println!("Creating new out_tx clone!");
                out_tx.clone()
            });
            out_tx2.send(elem * 10).unwrap()
        });
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

