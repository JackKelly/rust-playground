use crate::tracker::{Tracker, TrackerUsingBox};
use std::thread;

mod tracker;

struct Foo {
    a: usize,
    s: String,
    v: Vec<u8>,
}

impl Foo {
    fn new(i: usize) -> Self {
        Self {
            a: i,
            s: format!("{i}"),
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
    // perf stat (result from previous commit, using Tracker<Foo>)
    //     Performance counter stats for './target/release/page-faults':

    //     245.52 msec task-clock                       #    0.995 CPUs utilized
    //         9      context-switches                 #   36.657 /sec
    //         1      cpu-migrations                   #    4.073 /sec
    //     31,334      page-faults                      #  127.624 K/sec
    // 876,015,345      cycles                           #    3.568 GHz
    // 1,484,094,734      instructions                     #    1.69  insn per cycle
    // 289,492,663      branches                         #    1.179 G/sec
    // 1,988,980      branch-misses                    #    0.69% of all branches

    // 0.246733281 seconds time elapsed

    // 0.172706000 seconds user
    // 0.072295000 seconds sys

    // perf stat (using get_foo):
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
    // perf stat (result from previous commit, using TrackerWithBox<Foo>)
    //     Performance counter stats for './target/release/page-faults':

    //     335.88 msec task-clock                       #    0.992 CPUs utilized
    //         12      context-switches                 #   35.727 /sec
    //         0      cpu-migrations                   #    0.000 /sec
    //     35,240      page-faults                      #  104.918 K/sec
    // 1,219,417,371      cycles                           #    3.630 GHz
    // 2,192,701,091      instructions                     #    1.80  insn per cycle
    // 432,072,896      branches                         #    1.286 G/sec
    // 1,521,752      branch-misses                    #    0.35% of all branches

    // 0.338726103 seconds time elapsed

    // 0.150666000 seconds user
    // 0.182386000 seconds sys

    // perf stat (using get_foo):
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
