pub mod client;
mod config;
pub mod endpoints;
mod env;
mod error;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
pub use test_utils::TestContext;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
