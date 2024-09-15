use generate_consts::generate_consts;
use serde::{Deserialize, Serialize};

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
#[generate_consts(MyStruct)]
enum Inner {
    /// Docs about A
    A,
    /// Docs about B
    B,
    /// Docs about C
    C,
}

fn main() {}
