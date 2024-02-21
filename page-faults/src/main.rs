use crate::tracker::{Tracker, TrackerUsingBox};
use std::{ffi::CString, thread};

mod tracker;

struct Foo {
    a: usize,
    s: CString,
    v: Vec<u8>,
}

impl Foo {
    fn new(i: usize) -> Self {
        Self {
            a: i,
            s: CString::new(format!("{i}").as_bytes()).unwrap(),
            v: vec![i as u8],
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

fn tracker_without_box() {
    // perf stat (result from previous commit, using get_foo):
    //     Performance counter stats for './target/release/page-faults':

    //     232.65 msec task-clock                       #    0.998 CPUs utilized
    //         2      context-switches                 #    8.597 /sec
    //         0      cpu-migrations                   #    0.000 /sec
    //     31,334      page-faults                      #  134.686 K/sec
    // 853,568,165      cycles                           #    3.669 GHz
    // 1,472,453,604      instructions                     #    1.73  insn per cycle
    // 286,803,149      branches                         #    1.233 G/sec
    // 1,603,596      branch-misses                    #    0.56% of all branches

    // 0.233149301 seconds time elapsed

    // 0.171221000 seconds user
    // 0.059728000 seconds sys

    // perf stat (new result, using CString within Foo):
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
    // perf stat (result from previous commit, using get_foo):
    //     Performance counter stats for './target/release/page-faults':

    //     342.07 msec task-clock                       #    0.997 CPUs utilized
    //         3      context-switches                 #    8.770 /sec
    //         0      cpu-migrations                   #    0.000 /sec
    //     35,240      page-faults                      #  103.019 K/sec
    // 1,233,912,179      cycles                           #    3.607 GHz
    // 2,195,602,250      instructions                     #    1.78  insn per cycle
    // 432,470,106      branches                         #    1.264 G/sec
    // 1,517,877      branch-misses                    #    0.35% of all branches

    // 0.343104731 seconds time elapsed

    // 0.229724000 seconds user
    // 0.112846000 seconds sys

    // perf stat (new result, using CString within Foo):
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
    let t = thread::spawn(tracker_without_box);
    t.join().unwrap();
}
