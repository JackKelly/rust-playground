# ManuallyDrop

Quick experiment exploring [`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html).

To detect memory leaks:

```shell
sudo apt install heaptrack
cargo run
heaptrack target/debug/manuallydrop
```
