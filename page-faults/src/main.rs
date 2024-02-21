use tokio::sync::oneshot;

use crate::tracker::{Tracker, TrackerUsingBox};
use std::{
    ffi::CString,
    fmt::Error,
    sync::mpsc::{channel, Receiver},
    thread,
};

mod tracker;

struct Foo {
    a: usize,
    s: CString,
    v: Option<Result<Vec<u8>, Error>>,
    callback: Option<Box<dyn FnOnce(usize) + Send + Sync>>,
}

impl Foo {
    fn new(i: usize) -> Self {
        Self {
            a: i,
            s: CString::new(format!("{i}").as_bytes()).unwrap(),
            v: None,
            callback: Some(Box::new(move |_: usize| {})),
        }
    }

    fn get_multiple_foos(i: usize, n_foos: usize) -> Vec<Self> {
        (0..n_foos).map(|_| Foo::new(i)).collect()
    }

    fn get_multiple_boxed_foos(i: usize, n_foos: usize) -> Vec<Box<Self>> {
        (0..n_foos).map(|_| Box::new(Foo::new(i))).collect()
    }
}

const N: usize = 1_000_000;
const N_FOOS: usize = 4;

fn get_foo(i: usize) -> Foo {
    assert!(i < N);
    Foo::new(i)
}

fn modify_foo(f: &mut Foo, i: usize) {
    f.v = Some(Ok(vec![i as u8]));
}

fn tracker_without_box(rx: Receiver<Foo>, one_tx: oneshot::Sender<()>) {
    // perf stat (result from previous commit, with 10 "other" threads):
    //     Performance counter stats for './target/release/page-faults':

    //     700.25 msec task-clock                       #    1.609 CPUs utilized
    //     10,766      context-switches                 #   15.374 K/sec
    //         34      cpu-migrations                   #   48.554 /sec
    //     33,412      page-faults                      #   47.714 K/sec
    // 2,372,178,682      cycles                           #    3.388 GHz
    // 2,297,689,550      instructions                     #    0.97  insn per cycle
    // 443,988,408      branches                         #  634.039 M/sec
    // 4,847,589      branch-misses                    #    1.09% of all branches

    // 0.435135824 seconds time elapsed

    // 0.477327000 seconds user
    // 0.226730000 seconds sys

    // perf stat (where we use tokio::sync::oneshot::channel main awaits the channel rx):
    //     Performance counter stats for './target/release/page-faults':

    //     478.87 msec task-clock                       #    1.538 CPUs utilized
    //      8,580      context-switches                 #   17.917 K/sec
    //         28      cpu-migrations                   #   58.471 /sec
    //     33,415      page-faults                      #   69.779 K/sec
    // 1,650,791,795      cycles                           #    3.447 GHz
    // 2,152,699,601      instructions                     #    1.30  insn per cycle
    // 423,197,901      branches                         #  883.741 M/sec
    //  2,141,607      branch-misses                    #    0.51% of all branches

    // 0.311305315 seconds time elapsed

    // 0.360623000 seconds user
    // 0.118841000 seconds sys

    let mut tracker: Tracker<Foo> = Tracker::new(N);

    for _ in 0..N {
        let f = rx.recv().unwrap();
        let index = tracker.get_next_index().unwrap();
        tracker.put(index, f);
    }

    for i in 0..N {
        let f = tracker.as_mut(i).unwrap();
        modify_foo(f, i);
    }

    for i in 0..N {
        let f = tracker.remove(i).unwrap();
        let callback = f.callback.unwrap();
        callback(1)
    }
    println!("DONE!");
    one_tx.send(()).unwrap();
}

fn tracker_with_internal_boxes(rx: Receiver<Foo>, one_tx: oneshot::Sender<()>) {
    // perf stat (result from previous commit, with 10 "other" threads):
    //  Performance counter stats for './target/release/page-faults':

    //     663.82 msec task-clock                       #    1.569 CPUs utilized
    //     4,484      context-switches                 #    6.755 K/sec
    //     18      cpu-migrations                   #   27.116 /sec
    // 39,808      page-faults                      #   59.968 K/sec
    // 2,288,625,267      cycles                           #    3.448 GHz
    // 2,739,903,647      instructions                     #    1.20  insn per cycle
    // 546,358,750      branches                         #  823.053 M/sec
    // 4,702,359      branch-misses                    #    0.86% of all branches

    // 0.423028782 seconds time elapsed

    // 0.491972000 seconds user
    // 0.171990000 seconds sys

    // perf stat (where we use tokio::sync::oneshot::channel main awaits the channel rx):
    //     Performance counter stats for './target/release/page-faults':

    //     511.07 msec task-clock                       #    1.486 CPUs utilized
    //      4,829      context-switches                 #    9.449 K/sec
    //         15      cpu-migrations                   #   29.350 /sec
    //     42,126      page-faults                      #   82.428 K/sec
    // 1,788,990,220      cycles                           #    3.501 GHz
    // 2,674,420,345      instructions                     #    1.49  insn per cycle
    // 538,229,767      branches                         #    1.053 G/sec
    //  3,000,251      branch-misses                    #    0.56% of all branches

    // 0.344002367 seconds time elapsed

    // 0.338146000 seconds user
    // 0.173098000 seconds sys

    let mut tracker: TrackerUsingBox<Foo> = TrackerUsingBox::new(N);

    for _ in 0..N {
        let f = rx.recv().unwrap();
        let index = tracker.get_next_index().unwrap();
        tracker.put(index, f);
    }

    for i in 0..N {
        let f = tracker.as_mut(i).unwrap();
        modify_foo(f, i);
    }

    for i in 0..N {
        let f = tracker.remove(i).unwrap();
        let callback = f.callback.unwrap();
        callback(1)
    }
    println!("DONE!");

    one_tx.send(()).unwrap();
}

#[tokio::main]
async fn main() {
    let mut other_threads = Vec::new();
    for _ in 0..10 {
        other_threads.push(thread::spawn(|| {}));
    }

    let (tx, rx) = channel();
    let (one_tx, one_rx) = oneshot::channel();
    let t = thread::spawn(move || tracker_without_box(rx, one_tx));

    for i in 0..N {
        tx.send(get_foo(i)).unwrap();
    }

    one_rx.await.unwrap();

    // Finish.
    t.join().unwrap();
    other_threads.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });
}
