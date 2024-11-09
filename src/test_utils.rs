#![cfg(test)]

mod context;
mod function_id;
pub(crate) mod mock;
pub(crate) mod tracing;

pub(crate) use context::{test_context, TestContext};
pub(crate) use function_id::function_id;

pub trait MockDefault: Sized {
    fn mock_default() -> Self;
}
