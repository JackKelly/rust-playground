use rayon::prelude::*;

#[derive(Debug)]
enum Operation {
    Get(u8),
    EndOfGroup(u8),
}

fn main() {
    let ops = vec![
        Operation::Get(0),
        Operation::Get(1),
        Operation::Get(2),
        Operation::EndOfGroup(0),
        Operation::Get(10),
        Operation::Get(11),
        Operation::EndOfGroup(1),
    ];

    // Oops: Rayon may process these items out-of-order. So we might start loading group 1 before
    // we hit the EndOfGroup(0).
    ops.into_par_iter()
        .map(|op| {
            if matches!(op, Operation::Get { .. }) {
                Some(op)
            } else {
                None
            }
        })
        .while_some()
        .for_each(|op| println!("{:?}", op));
}
