#![deny(unused)]

struct Foo<T>(T);
impl<T> Foo<T> {
    const BAR: u32 = 3;
    fn new(x: T) -> Self {
        Foo(x)
    }
    fn get(&self) -> &T {
        &self.0
    }
}
fn bar() {}
useful::used!(let Foo::<()>::new, bar, let Foo::<()>::get, let Foo::<()>::BAR,);
useful::used_for_tests!(let Foo::<u32>::new);

fn main() {}
