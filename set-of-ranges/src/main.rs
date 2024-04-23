use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();

    set.insert(0..5);
    set.insert(5..10);

    println!("{:?}", set);
}
