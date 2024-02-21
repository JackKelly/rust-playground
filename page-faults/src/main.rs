use std::thread;

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

fn just_a_vec_of_vecs() {
    // perf stat (result from previous commit):
    //     Performance counter stats for './target/release/page-faults':

    //     320.26 msec task-clock                       #    0.998 CPUs utilized
    //         36      context-switches                 #  112.410 /sec
    //         4      cpu-migrations                   #   12.490 /sec
    //     29,380      page-faults                      #   91.739 K/sec
    // 1,135,048,532      cycles                           #    3.544 GHz
    // 2,208,078,923      instructions                     #    1.95  insn per cycle
    // 433,725,846      branches                         #    1.354 G/sec
    // 1,263,417      branch-misses                    #    0.29% of all branches

    // 0.321032341 seconds time elapsed

    // 0.228956000 seconds user
    // 0.088369000 seconds sys

    // perf stat (new result, for this commit, using Vec<Vec<Foo>>)
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

    // Put N Foos into a Vec:
    let vec: Vec<Vec<Foo>> = (0..N).map(|i| Foo::get_multiple_foos(i, N_FOOS)).collect();

    // Consume the vec:
    for f in vec.into_iter() {
        assert_eq!(f.len(), N_FOOS);
    }
    println!("DONE!");
}

fn vec_of_vec_of_boxes() {
    // perf stat (result from previous commit):
    //     Performance counter stats for './target/release/page-faults':

    //     419.55 msec task-clock                       #    0.998 CPUs utilized
    //         12      context-switches                 #   28.602 /sec
    //         1      cpu-migrations                   #    2.384 /sec
    //     33,287      page-faults                      #   79.341 K/sec
    // 1,483,567,429      cycles                           #    3.536 GHz
    // 2,819,406,676      instructions                     #    1.90  insn per cycle
    // 559,878,833      branches                         #    1.334 G/sec
    // 3,574,667      branch-misses                    #    0.64% of all branches

    // 0.420288255 seconds time elapsed

    // 0.318706000 seconds user
    // 0.100856000 seconds sys

    // perf stat (new result, for this commit, using Vec<Vec<Box<Foo>>>)
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

    // Put N Foos into a Vec:
    let vec: Vec<Vec<Box<Foo>>> = (0..N)
        .map(|i| Foo::get_multiple_boxed_foos(i, N_FOOS))
        .collect();

    // Consume the vec:
    for f in vec.into_iter() {
        assert_eq!(f.len(), N_FOOS);
    }
    println!("DONE!");
}

fn main() {
    let t = thread::spawn(vec_of_vec_of_boxes);
    t.join().unwrap();
}
