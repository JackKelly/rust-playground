use crate::tracker::{Tracker, TrackerUsingBox};
use std::{ffi::CString, fmt::Error, thread};

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

fn tracker_without_box() {
    // perf stat (result from previous commit, changing Foo.v from Vec<u8> to Option<Result<Vec<u8>>>):
    //     Performance counter stats for './target/release/page-faults':

    //     245.32 msec task-clock                       #    0.993 CPUs utilized
    //         26      context-switches                 #  105.982 /sec
    //         1      cpu-migrations                   #    4.076 /sec
    //     29,382      page-faults                      #  119.768 K/sec
    // 888,155,144      cycles                           #    3.620 GHz
    // 1,842,772,334      instructions                     #    2.07  insn per cycle
    // 365,392,692      branches                         #    1.489 G/sec
    // 257,191      branch-misses                    #    0.07% of all branches

    // 0.246958285 seconds time elapsed

    // 0.180939000 seconds user
    // 0.064334000 seconds sys

    // perf stat (new result, using separate for loop to modify foo):
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

    let mut tracker: Tracker<Foo> = Tracker::new(N);

    for i in 0..N {
        let index = tracker.get_next_index().unwrap();
        tracker.put(index, get_foo(i));
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

fn tracker_with_internal_boxes() {
    // perf stat (result from previous commit, changing Foo.v from Vec<u8> to Option<Result<Vec<u8>>>):
    //     Performance counter stats for './target/release/page-faults':

    //     373.11 msec task-clock                       #    0.995 CPUs utilized
    //         23      context-switches                 #   61.645 /sec
    //         1      cpu-migrations                   #    2.680 /sec
    //     35,243      page-faults                      #   94.458 K/sec
    // 1,343,844,820      cycles                           #    3.602 GHz
    // 2,418,324,295      instructions                     #    1.80  insn per cycle
    // 489,047,365      branches                         #    1.311 G/sec
    // 1,081,807      branch-misses                    #    0.22% of all branches

    // 0.375044581 seconds time elapsed

    // 0.272118000 seconds user
    // 0.100043000 seconds sys

    // perf stat (new result, using separate for loop to modify foo):
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

    let mut tracker: TrackerUsingBox<Foo> = TrackerUsingBox::new(N);

    for i in 0..N {
        let index = tracker.get_next_index().unwrap();
        tracker.put(index, get_foo(i));
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
    let t = thread::spawn(tracker_without_box);
    t.join().unwrap();
}
