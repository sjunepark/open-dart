/// The Params struct for OpenDart API endpoints
macro_rules! params {
    (
        $(
            $(#[$field_attr:meta])*
            $field_vis:vis $field_name:ident: $field_type:ty
        ),* $(,)?
    ) => {
        /// Documentation exists in each field's types
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
            derive_more::From,
            derive_more::Into,
            // serde
            serde::Serialize,
            serde::Deserialize,
            // builder
            derive_builder::Builder,
        )]
        #[builder(setter(into, strip_option), derive(Debug))]
        #[builder(build_fn(error = "crate::OpenDartError"))]
        #[display("{self:?}")]
        #[serde(deny_unknown_fields)]
        pub struct Params {
            #[builder(setter(skip))]
            crtfc_key: $crate::types::CrtfcKey,
            $(
                $(#[$field_attr])*
                $field_vis $field_name: $field_type
            ),*
        }
    };
}
pub(crate) use params;

#[cfg(test)]
mod tests {
    #[test]
    fn params_builder_should_with_all_fields_set() {
        params!(
            pub name: String,
            pub age: u16,
        );

        let name = "John Doe";
        let age = 42;
        let params = ParamsBuilder::default()
            .name(name)
            .age(age)
            .build()
            .expect("Failed to build Params");

        assert_eq!(
            params,
            Params {
                crtfc_key: Default::default(),
                name: name.to_string(),
                age,
            }
        );
    }

    #[test]
    fn params_builder_should_work_with_field_attributes_set() {
        params!(
            pub name: String,
            #[builder(setter(name = "renamed_age"))]
            pub age: u16,
        );

        let name = "John Doe";
        let age = 42;
        let params = ParamsBuilder::default()
            .name(name)
            .renamed_age(age)
            .build()
            .expect("Failed to build Params");

        assert_eq!(
            params,
            Params {
                crtfc_key: Default::default(),
                name: name.to_string(),
                age,
            }
        );
    }
}
