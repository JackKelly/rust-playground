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
}

impl Foo {
    fn new(i: usize) -> Self {
        Self {
            a: i,
            s: CString::new(format!("{i}").as_bytes()).unwrap(),
            v: None,
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
    // perf stat (result from previous commit, using separate for loop to modify foo):
    //     Performance counter stats for './target/release/page-faults':

    //     268.22 msec task-clock                       #    0.997 CPUs utilized
    //         2      context-switches                 #    7.456 /sec
    //         2      cpu-migrations                   #    7.456 /sec
    //     29,381      page-faults                      #  109.539 K/sec
    // 953,324,250      cycles                           #    3.554 GHz
    // 1,847,581,161      instructions                     #    1.94  insn per cycle
    // 372,301,578      branches                         #    1.388 G/sec
    // 1,408,990      branch-misses                    #    0.38% of all branches

    // 0.269130381 seconds time elapsed

    // 0.204217000 seconds user
    // 0.064068000 seconds sys

    // perf stat (new result, using a channel to send Foos to this thread):
    //     Performance counter stats for './target/release/page-faults':

    //     313.77 msec task-clock                       #    0.997 CPUs utilized
    //         12      context-switches                 #   38.245 /sec
    //         2      cpu-migrations                   #    6.374 /sec
    //     43,243      page-faults                      #  137.818 K/sec
    // 1,126,443,573      cycles                           #    3.590 GHz
    // 2,031,061,353      instructions                     #    1.80  insn per cycle
    // 401,767,162      branches                         #    1.280 G/sec
    // 1,081,191      branch-misses                    #    0.27% of all branches

    // 0.314785131 seconds time elapsed

    // 0.247828000 seconds user
    // 0.063955000 seconds sys

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
        tracker.remove(i).unwrap();
    }
    println!("DONE!");
}

fn tracker_with_internal_boxes(rx: Receiver<Foo>) {
    // perf stat (result from previous commit, using separate for loop to modify foo):
    //     Performance counter stats for './target/release/page-faults':

    //     384.64 msec task-clock                       #    0.996 CPUs utilized
    //         27      context-switches                 #   70.195 /sec
    //         0      cpu-migrations                   #    0.000 /sec
    //     35,241      page-faults                      #   91.620 K/sec
    // 1,381,068,337      cycles                           #    3.591 GHz
    // 2,499,073,364      instructions                     #    1.81  insn per cycle
    // 505,657,656      branches                         #    1.315 G/sec
    // 1,880,032      branch-misses                    #    0.37% of all branches

    // 0.386278528 seconds time elapsed

    // 0.255152000 seconds user
    // 0.129601000 seconds sys

    // perf stat (new result, using a channel to send Foos to this thread):
    //     Performance counter stats for './target/release/page-faults':

    //     415.84 msec task-clock                       #    0.996 CPUs utilized
    //         13      context-switches                 #   31.262 /sec
    //         1      cpu-migrations                   #    2.405 /sec
    //     49,104      page-faults                      #  118.084 K/sec
    // 1,483,189,562      cycles                           #    3.567 GHz
    // 2,623,045,903      instructions                     #    1.77  insn per cycle
    // 528,208,249      branches                         #    1.270 G/sec
    // 1,668,119      branch-misses                    #    0.32% of all branches

    // 0.417494504 seconds time elapsed

    // 0.291024000 seconds user
    // 0.123585000 seconds sys

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
        tracker.remove(i).unwrap();
    }
    println!("DONE!");
}

fn main() {
    let (tx, rx) = channel();

    for i in 0..N {
        tx.send(get_foo(i)).unwrap();
    }

    let t = thread::spawn(move || tracker_with_internal_boxes(rx));
    drop(tx);
    t.join().unwrap();
}
