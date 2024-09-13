use serde::{Deserialize, Serialize};
use test_variants::test_variants;

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, derive_more::AsRef,
)]
struct MyStruct(Inner);

impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize, derive_more::Display,
)]
#[display("{_variant}")]
#[test_variants(MyStruct)]
enum Inner {
    A,
    B,
    C,
}

fn main() {}
