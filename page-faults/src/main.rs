use io_uring::{opcode, squeue};
use tokio::sync::oneshot;

use std::{
    collections::VecDeque,
    sync::mpsc::{channel, Receiver},
    thread,
};
const N: usize = 1_000_000;
type InnerVec = Vec<Box<squeue::Entry>>;
// type InnerVec = Vec<squeue::Entry>;

fn worker_thread_function(rx: Receiver<usize>, oneshot_tx: oneshot::Sender<()>) {
    const SQ_RING_SIZE: usize = 14 * 3;
    let mut internal_queue: VecDeque<InnerVec> = VecDeque::with_capacity(SQ_RING_SIZE);
    let mut n = 0;

    while n < N {
        // Push Entries onto the internal queue:
        'inner: for _ in 0..SQ_RING_SIZE {
            if n >= N {
                break 'inner;
            }
            let msg = rx.recv().unwrap();
            let entries = create_sq_entries(msg);
            internal_queue.push_back(entries);
            n += 1;
        }

        // Consume Entries from the internal queue:
        while let Some(entries) = internal_queue.pop_front() {
            let entries = [*entries[0].clone(), *entries[1].clone()];
            assert_eq!(entries.len(), 2);
            let s = format!("{:?}, {:?}", entries[0], entries[1]);
            assert_eq!(s, "Entry { op_code: 0, flags: 0, user_data: 0 }, Entry { op_code: 0, flags: 0, user_data: 0 }");
        }
    }

    oneshot_tx.send(()).unwrap();
}

fn create_sq_entries(msg: usize) -> InnerVec {
    assert!(msg < N); // Just to make sure the compiler doesn't optimise away msg.

    vec![
        Box::new(opcode::Nop::new().build()),
        Box::new(opcode::Nop::new().build()),
    ]
    //vec![opcode::Nop::new().build(), opcode::Nop::new().build()]
}

#[tokio::main]
async fn main() {
    let (tx, rx) = channel();
    let (oneshot_tx, oneshot_rx) = oneshot::channel();
    let t = thread::spawn(move || worker_thread_function(rx, oneshot_tx));

    for i in 0..N {
        tx.send(i).unwrap();
    }

    // Finish.
    oneshot_rx.await.unwrap();
    t.join().unwrap();
}

// With Box:
// Performance counter stats for './target/release/page-faults':
// 690.53 msec task-clock                       #    1.040 CPUs utilized
//    471      context-switches                 #  682.084 /sec
//     13      cpu-migrations                   #   18.826 /sec
//  4,053      page-faults                      #    5.869 K/sec
// 2,462,191,190      cycles                           #    3.566 GHz
// 5,818,401,932      instructions                     #    2.36  insn per cycle
// 1,210,713,595      branches                         #    1.753 G/sec
// 5,558,308      branch-misses                    #    0.46% of all branches

// 0.663811234 seconds time elapsed

// 0.675375000 seconds user
// 0.011918000 seconds sys

/////////////////////////////////////////////////////////////////////
// Without Box:
// Performance counter stats for './target/release/page-faults':
// 591.49 msec task-clock                       #    1.045 CPUs utilized
//    498      context-switches                 #  841.940 /sec
//     13      cpu-migrations                   #   21.978 /sec
//  4,077      page-faults                      #    6.893 K/sec
// 2,113,414,848      cycles                           #    3.573 GHz
// 5,092,619,061      instructions                     #    2.41  insn per cycle
// 1,045,961,482      branches                         #    1.768 G/sec
// 5,239,599      branch-misses                    #    0.50% of all branches

// 0.565925373 seconds time elapsed

// 0.582248000 seconds user
// 0.007976000 seconds sys
