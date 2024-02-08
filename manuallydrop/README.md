# ManuallyDrop

Quick experiment exploring [`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html).

To detect memory leaks:

1. `sudo apt install heaptrack`
2. `cargo run`
3. `heaptrack target/debug/manuallydrop`
