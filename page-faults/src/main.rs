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
    let mut f = Foo::new(i);
    f.v = Some(Ok(vec![i as u8]));
    f
}

fn tracker_without_box() {
    // perf stat (result from previous commit, using CString within Foo):
    //     Performance counter stats for './target/release/page-faults':

    //     252.80 msec task-clock                       #    0.998 CPUs utilized
    //         2      context-switches                 #    7.911 /sec
    //         0      cpu-migrations                   #    0.000 /sec
    //     29,380      page-faults                      #  116.218 K/sec
    // 897,499,932      cycles                           #    3.550 GHz
    // 1,787,229,694      instructions                     #    1.99  insn per cycle
    // 358,050,110      branches                         #    1.416 G/sec
    // 876,981      branch-misses                    #    0.24% of all branches

    // 0.253266065 seconds time elapsed

    // 0.183792000 seconds user
    // 0.067923000 seconds sys

    // perf stat (new result, changing Foo.v from Vec<u8> to Option<Result<Vec<u8>>>):
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

    let mut tracker: Tracker<Foo> = Tracker::new(N);

    for i in 0..N {
        let index = tracker.get_next_index().unwrap();
        tracker.put(index, get_foo(i));
    }

    for i in 0..N {
        tracker.remove(i).unwrap();
    }
    println!("DONE!");
}

fn tracker_with_internal_boxes() {
    // perf stat (result from previous commit, using CString within Foo):
    //     Performance counter stats for './target/release/page-faults':

    //     355.13 msec task-clock                       #    0.996 CPUs utilized
    //         6      context-switches                 #   16.895 /sec
    //         2      cpu-migrations                   #    5.632 /sec
    //     35,240      page-faults                      #   99.230 K/sec
    // 1,262,708,379      cycles                           #    3.556 GHz
    // 2,482,387,004      instructions                     #    1.97  insn per cycle
    // 500,601,903      branches                         #    1.410 G/sec
    // 734,245      branch-misses                    #    0.15% of all branches

    // 0.356446790 seconds time elapsed

    // 0.282799000 seconds user
    // 0.071695000 seconds sys

    // perf stat (new result, changing Foo.v from Vec<u8> to Option<Result<Vec<u8>>>):
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

    let mut tracker: TrackerUsingBox<Foo> = TrackerUsingBox::new(N);

    for i in 0..N {
        let index = tracker.get_next_index().unwrap();
        tracker.put(index, get_foo(i));
    }

    for i in 0..N {
        tracker.remove(i).unwrap();
    }
    println!("DONE!");
}

fn main() {
    let t = thread::spawn(tracker_with_internal_boxes);
    t.join().unwrap();
}
