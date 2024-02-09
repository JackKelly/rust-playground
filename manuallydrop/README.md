# ManuallyDrop

Quick experiment exploring [`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html) and `Box::into_raw`.

It turns out that [my original attempt to use `ManuallyDrop`](https://github.com/JackKelly/rust-playground/blob/d5688a65bd01fa20d551637f791fb54aaf0bc009/manuallydrop/src/main.rs) to keep an object alive after it goes out of scope, failed for the `String` member of `Foo` when compiled in `release` mode. I tried all the different combinations of `ManuallyDrop` and `Pin` that I could think of, but none worked! The only thing I could get to work was using `Box::into_raw`.

To detect memory leaks:

```shell
cargo build --release
sudo apt install heaptrack
heaptrack target/release/manuallydrop
```
