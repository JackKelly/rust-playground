struct Foo<'life1> {
    a: &'life1 str,
    b: &'life1 str,
}

impl<'life1> Foo<'life1> {
    fn new(a: &'life1 str, b: &'life1 str) -> Self {
        Self { a, b }
    }

    fn longest(&self) -> &'life1 str {
        if self.a.len() > self.b.len() {
            self.a
        } else {
            self.b
        }
    }
}

fn main() {
    let foo;

    let s1 = "hello".to_string();
    let s2 = "world!".to_string();

    foo = Foo::new(&s1, &s2);

    let longest = foo.longest();
    println!("{longest}");

    let v = vec![&s1, &s2];
    println!("{v:?}");
}
