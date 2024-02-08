# ManuallyDrop

Quick experiment exploring [`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html).

To detect memory leaks:

```shell
cargo build
sudo apt install heaptrack
heaptrack target/debug/manuallydrop
```
