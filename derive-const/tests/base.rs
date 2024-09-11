use derive_const::impl_const;

#[impl_const(variants = Inner)]
struct MyStruct(Inner);

enum Inner {
    A,
    B,
    C,
    D,
}

fn main() {}
