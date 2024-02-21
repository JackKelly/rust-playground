use std::thread;

struct Foo {
    a: usize,
    s: String,
    v: Vec<u8>,
}

const N: usize = 1_000_000;

fn just_a_vec() {
    // perf stat (before Foo included `v`):
    //     Performance counter stats for './target/release/page-faults':
    //
    //     134.22 msec task-clock                       #    0.997 CPUs utilized
    //          1      context-switches                 #    7.451 /sec
    //          0      cpu-migrations                   #    0.000 /sec
    //     15,701      page-faults                      #  116.981 K/sec
    // 491,428,003      cycles                           #    3.661 GHz
    // 1,280,540,225      instructions                     #    2.61  insn per cycle
    // 247,147,881      branches                         #    1.841 G/sec
    //    763,384      branch-misses                    #    0.31% of all branches
    //
    // 0.134606829 seconds time elapsed
    //
    // 0.110119000 seconds user
    // 0.024470000 seconds sys

    // perf stat (after Foo included `v`):
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

    // Put N Foos into a Vec:
    let vec: Vec<Foo> = (0..N)
        .map(|i| Foo {
            a: i,
            s: format!("{i}"),
            v: vec![i as u8],
        })
        .collect();

    // Consume the vec:
    for (i, f) in vec.into_iter().enumerate() {
        assert_eq!(f.a, i);
        assert_eq!(f.s, format!("{i}"));
        assert_eq!(f.v, vec![i as u8]);
    }
    println!("DONE!");
}

fn vec_of_boxes() {
    // perf stat (before Foo included `v`):
    //     Performance counter stats for './target/release/page-faults':
    //
    //     191.17 msec task-clock                       #    0.996 CPUs utilized
    //         28      context-switches                 #  146.464 /sec
    //          0      cpu-migrations                   #    0.000 /sec
    //     21,562      page-faults                      #  112.788 K/sec
    // 693,299,236      cycles                           #    3.627 GHz
    // 1,613,260,564      instructions                     #    2.33  insn per cycle
    // 324,287,992      branches                         #    1.696 G/sec
    //  1,230,289      branch-misses                    #    0.38% of all branches
    //
    // 0.191929539 seconds time elapsed
    //
    // 0.147424000 seconds user
    // 0.043829000 seconds sys`

    // perf stat (after Foo included `v`):
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

    // Put N Foos into a Vec:
    let vec: Vec<Box<Foo>> = (0..N)
        .map(|i| {
            Box::new(Foo {
                a: i,
                s: format!("{i}"),
                v: vec![i as u8],
            })
        })
        .collect();

    // Consume the vec:
    for (i, f) in vec.into_iter().enumerate() {
        assert_eq!(f.a, i);
        assert_eq!(f.s, format!("{i}"));
        assert_eq!(f.v, vec![i as u8]);
    }
    println!("DONE!");
}

fn main() {
    let t = thread::spawn(vec_of_boxes);
    t.join().unwrap();
}
