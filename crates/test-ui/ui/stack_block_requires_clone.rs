use block2::StackBlock;

struct Foo;

fn main() {
    let foo = Foo;
    let _ = StackBlock::new(move || {
        let _ = &foo;
    });
}
