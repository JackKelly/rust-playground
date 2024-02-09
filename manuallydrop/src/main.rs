#[derive(Debug)]
struct Foo {
    i: u64,
    s: String,
}

impl Drop for Foo {
    fn drop(&mut self) {
        // Implement `drop` so we can see when `Foo` is dropped!
        println!("DROP FOO!");
    }
}

fn main() {
    let ptr;

    {
        let inner_foo = Box::new(Foo {
            i: 42,
            s: "hello!".to_string(),
        });
        println!(" original: {:?}", *inner_foo);

        ptr = Box::into_raw(inner_foo);
        unsafe {
            println!("inner ptr: {:?}", *ptr);
        }
    }

    let outer_foo: Box<Foo>;
    unsafe {
        println!("outer ptr: {:?}", *ptr);
        outer_foo = Box::from_raw(ptr);
    }
    println!("outer_foo: {:?}", outer_foo);
} // `outer_foo` is dropped here.
