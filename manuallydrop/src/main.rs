use std::mem::ManuallyDrop;

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
        let mut inner_foo = ManuallyDrop::new(Foo {
            i: 42,
            s: "hello!".to_string(),
        });
        println!(" original: {:?}", *inner_foo);

        ptr = &mut inner_foo as *mut ManuallyDrop<Foo>;
        unsafe {
            println!("inner ptr: {:?}", **ptr);
        }

        // If we don't use `ManuallyDrop` then `foo` is dropped here.
    }

    let outer_foo: Foo;
    unsafe {
        println!("outer ptr: {:?}", **ptr);

        // If the following line is commented out then `heaptrack` detects a memory leak
        // and `drop` isn't called.
        outer_foo = ManuallyDrop::take(&mut *ptr);

        // Instead of `ManuallyDrop::take`, we could call:
        // ManuallyDrop::drop(&mut *ptr);
    }
    println!("outer foo: {:?}", outer_foo);
} // `outer_foo` is dropped here.
