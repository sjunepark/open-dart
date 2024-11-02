macro_rules! assert_impl_commons_without_default {
    ($type:ty) => {
        static_assertions::assert_impl_all!($type: Clone, Eq, PartialEq, Ord, PartialOrd, std::fmt::Debug, std::fmt::Display, Send, Sync, serde::Serialize, serde::de::DeserializeOwned);
    };
}

pub(crate) use assert_impl_commons_without_default;
