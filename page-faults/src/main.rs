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

fn tracker_without_box(rx: Receiver<Foo>) {
    // perf stat (result from previous commit, add callback to Foo):
    //     Performance counter stats for './target/release/page-faults':

    //     578.30 msec task-clock                       #    1.581 CPUs utilized
    //     10,736      context-switches                 #   18.565 K/sec
    //         13      cpu-migrations                   #   22.480 /sec
    //     33,391      page-faults                      #   57.740 K/sec
    // 1,929,122,730      cycles                           #    3.336 GHz
    // 2,241,697,839      instructions                     #    1.16  insn per cycle
    // 436,382,914      branches                         #  754.600 M/sec
    // 3,235,032      branch-misses                    #    0.74% of all branches

    // 0.365745866 seconds time elapsed

    // 0.388474000 seconds user
    // 0.194237000 seconds sys

    // perf stat (new result, with 10 "other" threads):
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

    let mut tracker: Tracker<Foo> = Tracker::new(N);

    for f in rx.iter() {
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
}

fn tracker_with_internal_boxes(rx: Receiver<Foo>) {
    // perf stat (result from previous commit, add callback to Foo):
    //     Performance counter stats for './target/release/page-faults':

    //     559.79 msec task-clock                       #    1.529 CPUs utilized
    //      4,359      context-switches                 #    7.787 K/sec
    //         14      cpu-migrations                   #   25.009 /sec
    //     41,593      page-faults                      #   74.301 K/sec
    // 1,914,205,630      cycles                           #    3.419 GHz
    // 2,704,023,649      instructions                     #    1.41  insn per cycle
    // 540,543,477      branches                         #  965.611 M/sec
    //  2,092,312      branch-misses                    #    0.39% of all branches

    // 0.366229602 seconds time elapsed

    // 0.372921000 seconds user
    // 0.188555000 seconds sys

    // perf stat (new result, with 10 "other" threads):
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

    let mut tracker: TrackerUsingBox<Foo> = TrackerUsingBox::new(N);

    for f in rx.iter() {
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
}

#[tokio::main]
async fn main() {
    let mut other_threads = Vec::new();
    for _ in 0..10 {
        other_threads.push(thread::spawn(|| {}));
    }

    let (tx, rx) = channel();
    let t = thread::spawn(move || tracker_with_internal_boxes(rx));

    for i in 0..N {
        tx.send(get_foo(i)).unwrap();
    }

    // Finish.
    drop(tx);
    t.join().unwrap();
    other_threads.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });
}
