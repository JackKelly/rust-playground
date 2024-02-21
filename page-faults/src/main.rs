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

fn tracker_without_box() {
    // perf stat (result from previous commit):
    //     Performance counter stats for './target/release/page-faults':

    //     1,131.38 msec task-clock                       #    0.996 CPUs utilized
    //         38      context-switches                 #   33.587 /sec
    //         2      cpu-migrations                   #    1.768 /sec
    //     127,037      page-faults                      #  112.285 K/sec
    // 4,075,400,737      cycles                           #    3.602 GHz
    // 7,178,461,174      instructions                     #    1.76  insn per cycle
    // 1,408,050,445      branches                         #    1.245 G/sec
    // 5,003,878      branch-misses                    #    0.36% of all branches

    // 1.135567781 seconds time elapsed

    // 0.713242000 seconds user
    // 0.414397000 seconds sys

    // perf stat (new result, for this commit, using Tracker<Foo>)
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

    let mut tracker: Tracker<Foo> = Tracker::new(N);

    for i in 0..N {
        let index = tracker.get_next_index().unwrap();
        tracker.put(index, Foo::new(i));
    }

    for i in 0..N {
        tracker.remove(i).unwrap();
    }
    println!("DONE!");
}

fn tracker_with_internal_boxes() {
    // perf stat (result from previous commit):
    //     Performance counter stats for './target/release/page-faults':

    //     1,456.03 msec task-clock                       #    0.998 CPUs utilized
    //         12      context-switches                 #    8.242 /sec
    //         2      cpu-migrations                   #    1.374 /sec
    //     142,664      page-faults                      #   97.981 K/sec
    // 5,256,964,184      cycles                           #    3.610 GHz
    // 8,986,730,173      instructions                     #    1.71  insn per cycle
    // 1,794,199,600      branches                         #    1.232 G/sec
    // 6,784,732      branch-misses                    #    0.38% of all branches

    // 1.459089706 seconds time elapsed

    // 1.017636000 seconds user
    // 0.434989000 seconds sys

    // perf stat (new result, for this commit, using TrackerWithBox<Foo>)
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

    let mut tracker: TrackerUsingBox<Foo> = TrackerUsingBox::new(N);

    for i in 0..N {
        let index = tracker.get_next_index().unwrap();
        tracker.put(index, Foo::new(i));
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
