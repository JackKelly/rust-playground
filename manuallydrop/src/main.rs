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
        let mut foo = ManuallyDrop::new(Foo {
            i: 42,
            s: "hello!".to_string(),
        });
        println!("original: {:?}", *foo);

        ptr = &mut foo as *mut ManuallyDrop<Foo>;
        unsafe {
            println!("   inner: {:?}", **ptr);
        }

        // If we don't use `ManuallyDrop` then `foo` is dropped here.
    }

    unsafe {
        println!("   outer: {:?}", **ptr);

        // If the following line is commented out then `heaptrack` detects a memory leak
        // and `drop` isn't called.
        ManuallyDrop::drop(&mut *ptr);
    }
}
