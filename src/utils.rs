macro_rules! derive_newtype {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident($(#[$inner_attr:meta])* $inner_type:ty);
    ) => {
        #[derive(
            std::fmt::Debug,
            Clone,
            Eq,
            PartialEq,
            Ord,
            PartialOrd,
            Hash,
            // derive_more
            derive_more::AsRef,
            derive_more::Display,
            derive_more::From,
            derive_more::Into,
            // serde
            serde::Serialize,
            serde::Deserialize,
        )]
        #[cfg_attr(
            feature = "diesel_newtype",
            derive(diesel_derive_newtype::DieselNewType)
        )]
        $(#[$attr])*
        $vis struct $name($(#[$inner_attr])* $inner_type);
    };
}

pub(crate) use derive_newtype;

macro_rules! derive_enum {
    (
        $(#[$attr:meta])*
        $vis:vis enum $enum_name:ident {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident
            ),* $(,)?
        }
    ) => {
        #[derive(
            std::fmt::Debug,
            Clone,
            Eq,
            PartialEq,
            Ord,
            PartialOrd,
            Hash,
            // derive_more
            derive_more::Display,
            // serde
            serde::Serialize,
            serde::Deserialize,
        )]
        $(#[$attr])*
        $vis enum $enum_name {
            $(
                $(#[$variant_attr])*
                $variant
            ),*
        }
    };
}

pub(crate) use derive_enum;

#[cfg(test)]
mod tests {
    #[test]
    fn derive_newtype_works() {
        derive_newtype! {
            /// My new type
            pub struct MyNewType(String);
        }

        let my_new_type = MyNewType("Hello".to_string());
        assert_eq!(my_new_type.to_string(), "Hello");
    }

    #[test]
    fn derive_enum_works() {
        derive_enum! {
            /// My enum
            pub enum MyEnum {
                /// Variant A
                A,
            }
        }

        let my_enum = MyEnum::A;
        assert_eq!(my_enum, MyEnum::A);
    }
}
