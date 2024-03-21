use anyhow;
use std::io;

fn main() {
    let err = io::Error::new(io::ErrorKind::NotFound, "test error");
    let err = anyhow::Error::new(err)
        .context("conext 1")
        .context("context 2");
    println!("{err:?}");
}
