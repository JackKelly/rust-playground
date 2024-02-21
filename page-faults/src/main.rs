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
}

const N: usize = 1_000_000;

fn just_a_vec() {
    // perf stat (result from previous commit):
    //     Performance counter stats for './target/release/page-faults':

    //     283.65 msec task-clock                       #    0.999 CPUs utilized
    //         2      context-switches                 #    7.051 /sec
    //         0      cpu-migrations                   #    0.000 /sec
    //     29,380      page-faults                      #  103.577 K/sec
    // 1,019,463,064      cycles                           #    3.594 GHz
    // 2,189,885,518      instructions                     #    2.15  insn per cycle
    // 433,290,818      branches                         #    1.528 G/sec
    // 1,500,755      branch-misses                    #    0.35% of all branches
    //
    // 0.284074219 seconds time elapsed
    //
    // 0.219621000 seconds user
    // 0.063889000 seconds sys

    // perf stat (new result, for this commit, using a constructor function)
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

    // Put N Foos into a Vec:
    let vec: Vec<Foo> = (0..N).map(Foo::new).collect();

    // Consume the vec:
    for (i, f) in vec.into_iter().enumerate() {
        assert_eq!(f.a, i);
        assert_eq!(f.s, format!("{i}"));
        assert_eq!(f.v, vec![i as u8]);
    }
    println!("DONE!");
}

fn vec_of_boxes() {
    // perf stat (result from previous commit):
    //     Performance counter stats for './target/release/page-faults':

    //     382.32 msec task-clock                       #    0.997 CPUs utilized
    //         21      context-switches                 #   54.928 /sec
    //         0      cpu-migrations                   #    0.000 /sec
    //     33,287      page-faults                      #   87.066 K/sec
    // 1,364,747,440      cycles                           #    3.570 GHz
    // 2,819,102,900      instructions                     #    2.07  insn per cycle
    // 560,198,170      branches                         #    1.465 G/sec
    // 1,050,852      branch-misses                    #    0.19% of all branches
    //
    // 0.383418122 seconds time elapsed
    //
    // 0.261419000 seconds user
    // 0.120655000 seconds sys

    // perf stat (new result, for this commit, using a constructor function)
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

    // Put N Foos into a Vec:
    let vec: Vec<Box<Foo>> = (0..N).map(|i| Box::new(Foo::new(i))).collect();

    // Consume the vec:
    for (i, f) in vec.into_iter().enumerate() {
        assert_eq!(f.a, i);
        assert_eq!(f.s, format!("{i}"));
        assert_eq!(f.v, vec![i as u8]);
    }
    println!("DONE!");
}

fn main() {
    let t = thread::spawn(just_a_vec);
    t.join().unwrap();
}
