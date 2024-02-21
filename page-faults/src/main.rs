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
    // perf stat (result from previous commit, moving loop in main to after thread::spawn):
    //     Performance counter stats for './target/release/page-faults':

    //     594.65 msec task-clock                       #    1.590 CPUs utilized
    //     10,199      context-switches                 #   17.151 K/sec
    //         23      cpu-migrations                   #   38.678 /sec
    //     29,440      page-faults                      #   49.508 K/sec
    // 1,992,626,918      cycles                           #    3.351 GHz
    // 2,263,279,897      instructions                     #    1.14  insn per cycle
    // 440,862,975      branches                         #  741.386 M/sec
    // 2,596,662      branch-misses                    #    0.59% of all branches

    // 0.373985917 seconds time elapsed

    // 0.398951000 seconds user
    // 0.199475000 seconds sys

    // perf stat (new result, add callback to Foo):
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
    // perf stat (result from previous commit, moving loop in main to after thread::spawn):
    //     Performance counter stats for './target/release/page-faults':

    //     497.91 msec task-clock                       #    1.536 CPUs utilized
    //     4,158      context-switches                 #    8.351 K/sec
    //         12      cpu-migrations                   #   24.101 /sec
    //     35,550      page-faults                      #   71.399 K/sec
    // 1,748,488,569      cycles                           #    3.512 GHz
    // 2,600,010,625      instructions                     #    1.49  insn per cycle
    // 525,993,682      branches                         #    1.056 G/sec
    // 3,503,730      branch-misses                    #    0.67% of all branches

    // 0.324156369 seconds time elapsed

    // 0.346070000 seconds user
    // 0.150642000 seconds sys

    // perf stat (new result, add callback to Foo):
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
    let (tx, rx) = channel();
    let t = thread::spawn(move || tracker_without_box(rx));

    for i in 0..N {
        tx.send(get_foo(i)).unwrap();
    }

    // Finish.
    drop(tx);
    t.join().unwrap();
}
