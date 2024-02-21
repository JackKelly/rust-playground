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
    // perf stat (result from previous commit, adding tokio::main to main function):
    //     Performance counter stats for './target/release/page-faults':

    //     437.33 msec task-clock                       #    1.000 CPUs utilized
    //         46      context-switches                 #  105.185 /sec
    //         8      cpu-migrations                   #   18.293 /sec
    //     49,160      page-faults                      #  112.410 K/sec
    // 1,534,198,396      cycles                           #    3.508 GHz
    // 2,582,509,500      instructions                     #    1.68  insn per cycle
    // 522,394,062      branches                         #    1.195 G/sec
    // 2,458,108      branch-misses                    #    0.47% of all branches

    // 0.437383174 seconds time elapsed

    // 0.295615000 seconds user
    // 0.139817000 seconds sys

    // perf stat (new result, moving loop in main to after thread::spawn):
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
    // perf stat (result from previous commit, adding tokio::main to main function):
    //     Performance counter stats for './target/release/page-faults':

    //     432.97 msec task-clock                       #    0.998 CPUs utilized
    //         75      context-switches                 #  173.223 /sec
    //         10      cpu-migrations                   #   23.096 /sec
    //     49,158      page-faults                      #  113.537 K/sec
    // 1,528,135,578      cycles                           #    3.529 GHz
    // 2,583,831,290      instructions                     #    1.69  insn per cycle
    // 522,762,149      branches                         #    1.207 G/sec
    // 1,622,149      branch-misses                    #    0.31% of all branches

    // 0.433699119 seconds time elapsed

    // 0.309037000 seconds user
    // 0.121988000 seconds sys

    // perf stat (new result, moving loop in main to after thread::spawn):
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

#[tokio::main]
async fn main() {
    let (tx, rx) = channel();
    let t = thread::spawn(move || tracker_with_internal_boxes(rx));

    for i in 0..N {
        tx.send(get_foo(i)).unwrap();
    }

    // Finish.
    drop(tx);
    t.join().unwrap();
}
