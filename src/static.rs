#[macro_export]
macro_rules! assert_impl_all_commons {
    ($type:ty) => {
        ::static_assertions::assert_impl_all!(
            $type:
            Send,
            Sync,
            Clone,
            std::fmt::Debug,
            std::fmt::Display,
            serde::Serialize,
            serde::de::DeserializeOwned,
            PartialEq,
            PartialOrd,
        );
    };
}
