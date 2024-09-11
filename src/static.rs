#[macro_export]
macro_rules! assert_impl_all_commons {
    ($type:ty) => {
        ::static_assertions::assert_impl_all!(
            $type:
            Clone,
            Eq,
            PartialEq,
            Ord,
            PartialOrd,
            std::fmt::Debug,
            std::fmt::Display,
            Default,
            Send,
            Sync,
            serde::Serialize,
            serde::de::DeserializeOwned,
        );
    };
}

// Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Default, Send, Sync, Serialize, DeserializeOwned
