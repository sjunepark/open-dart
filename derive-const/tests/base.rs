use derive_const::Const;

#[derive(Const)]
struct MyStruct(Inner);

enum Inner {
    A,
    B,
    C,
    D,
}

impl MyStruct {
    const A: Self = Self(Inner::A);
    const B: Self = Self(Inner::B);
    const C: Self = Self(Inner::C);
    const D: Self = Self(Inner::D);
}

fn main() {}
