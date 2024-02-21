struct Foo {
    a: usize,
    s: String,
}

const N: usize = 1_000_000;

fn just_a_vec() {
    // perf stat:
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

    // Put N Foos into a Vec:
    let vec: Vec<Foo> = (0..N)
        .map(|i| Foo {
            a: i,
            s: format!("{i}"),
        })
        .collect();

    // Consume the vec:
    for (i, f) in vec.into_iter().enumerate() {
        assert_eq!(f.a, i);
        assert_eq!(f.s, format!("{i}"));
    }
}

fn vec_of_boxes() {
    // perf stat:
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

    // Put N Foos into a Vec:
    let vec: Vec<Box<Foo>> = (0..N)
        .map(|i| {
            Box::new(Foo {
                a: i,
                s: format!("{i}"),
            })
        })
        .collect();

    // Consume the vec:
    for (i, f) in vec.into_iter().enumerate() {
        assert_eq!(f.a, i);
        assert_eq!(f.s, format!("{i}"));
    }
}

fn main() {
    // just_a_vec();
    vec_of_boxes()
}
