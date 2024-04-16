use rayon::prelude::*;

#[derive(Debug)]
enum Operation {
    Get(u8),
}

fn main() {
    let (completion_tx, completion_rx) = crossbeam::channel::bounded(4);

    {
        // Send the first group of operations:
        let (submission_tx, submission_rx) = crossbeam::channel::bounded(4);
        let (inner_submission_tx_0, inner_submission_rx_0) = crossbeam::channel::bounded(4);
        vec![Operation::Get(1), Operation::Get(2), Operation::Get(3)]
            .into_iter()
            .for_each(|op| inner_submission_tx_0.send(op).unwrap());
        submission_tx.send(inner_submission_rx_0).unwrap();

        drop(inner_submission_tx_0);
        drop(submission_tx);

        submission_rx.into_iter().par_bridge().for_each(|inner| {
            let out = inner.into_iter().par_bridge().reduce(
                || Operation::Get(0), // Identity function.
                |Operation::Get(a), Operation::Get(b)| Operation::Get(a + b),
            );
            completion_tx.send(out).unwrap();
        });
    }

    drop(completion_tx);

    completion_rx.into_iter().for_each(|op| println!("{op:?}"));
}
