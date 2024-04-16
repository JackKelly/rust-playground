use rayon::prelude::*;

#[derive(Debug)]
enum Operation {
    Get(u8),
}

fn main() {
    let (completion_tx, completion_rx) = crossbeam::channel::bounded(4);

    {
        let (submission_tx, submission_rx) = crossbeam::channel::bounded(4);

        // Send the first group of operations:
        let (inner_submission_tx_0, inner_submission_rx_0) = crossbeam::channel::bounded(4);
        vec![
            Operation::Get(1),
            Operation::Get(2),
            Operation::Get(3),
            Operation::Get(4),
        ]
        .into_iter()
        .for_each(|op| inner_submission_tx_0.send(op).unwrap());
        submission_tx.send(inner_submission_rx_0).unwrap();
        drop(inner_submission_tx_0);

        // Send the second group of operations:
        let (inner_submission_tx_1, inner_submission_rx_1) = crossbeam::channel::bounded(4);
        vec![
            Operation::Get(6),
            Operation::Get(7),
            Operation::Get(8),
            Operation::Get(9),
        ]
        .into_iter()
        .for_each(|op| inner_submission_tx_1.send(op).unwrap());
        submission_tx.send(inner_submission_rx_1).unwrap();
        drop(inner_submission_tx_1);

        // Hang up the submission_tx, otherwise we'll never finish!
        drop(submission_tx);

        // "Process" the submission queue, and send data to the completion queue:
        submission_rx.into_iter().par_bridge().for_each(|inner| {
            let (inner_completion_tx, inner_completion_rx) =
                crossbeam::channel::bounded::<Operation>(4);
            inner
                .into_iter()
                .par_bridge()
                .for_each(|Operation::Get(x)| {
                    inner_completion_tx.send(Operation::Get(x * 10)).unwrap()
                });
            completion_tx.send(inner_completion_rx).unwrap();
        });
    }

    drop(completion_tx);

    completion_rx.into_iter().for_each(|inner| {
        println!("GROUP:");
        inner.into_iter().for_each(|op| println!("{op:?}"));
    });
}
