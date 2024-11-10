// region: Request Params

/// The Params struct for OpenDart API endpoints
macro_rules! params {
    (
        $(
            $(#[$field_attr:meta])*
            $field_vis:vis $field_name:ident: $field_type:ty
        ),* $(,)?
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
            derive_more::From,
            derive_more::Into,
            // serde
            serde::Serialize,
            serde::Deserialize,
            // builder
            derive_builder::Builder,
            // validator
            validator::Validate,
        )]
        #[builder(setter(into, strip_option), derive(Debug))]
        #[builder(build_fn(error = "crate::OpenDartError"))]
        #[display("{self:?}")]
        #[serde(deny_unknown_fields)]
        pub struct Params {
            // todo: Add documentation in `.md` files
            // $(
            //     #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/", stringify!($field_name), ".md"))]
            // )*
            #[builder(setter(skip), default = "Self::default_crtfc_key()")]
            #[validate(custom(function = "crate::validate::fields::crtfc_key"))]
            crtfc_key: String,
            $(
                $(#[$field_attr])*
                $field_vis $field_name: $field_type
            ),*
        }

        impl ParamsBuilder {
            fn default_crtfc_key() -> String {
                std::env::var("OPEN_DART_API_KEY")
                .expect("OPEN_DART_API_KEY must be set as an environment variable")
            }
        }
    };
}

pub(crate) use params;

// endregion: Request Params

// region: Response Body

macro_rules! derive_common {
    (
        $(#[$struct_attr:meta])*
        $struct_name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_name:ident: $field_type:ty
            ),* $(,)?
        }
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
        )]
        #[display("{self:?}")]
        #[serde(deny_unknown_fields)]
        $(#[$struct_attr])*
        pub struct $struct_name {
            $(
                $(#[$field_attr])*
                $field_vis $field_name: $field_type
            ),*
        }
    };
}
pub(crate) use derive_common;

/// The JSON body struct for an OpenDart API response
macro_rules! json_body {
    (
        $struct_name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_name:ident: $field_type:ty
            ),* $(,)?
        }
    ) => {
        $crate::endpoints::macros::derive_common!($struct_name {
            status: String,
            message: String,
            $($field_vis $field_name: $field_type),*
        });

        impl $crate::endpoints::ResponseCheck for $struct_name {
            fn is_success(&self) -> Result<(), $crate::error::MessageError> {
                    if self.status == "000" {
                        Ok(())
                    } else {
                        Err($crate::error::MessageError {
                            message: $crate::endpoints::Message {
                                status: self.status.to_string(),
                                message: "".to_string(),
                            },
                        })
                    }
            }
        }
    };
}

pub(crate) use json_body;

// endregion: Response Body

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
                crtfc_key: std::env::var("OPEN_DART_API_KEY")
                    .expect("OPEN_DART_API_KEY must be set as an environment variable"),
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
                crtfc_key: std::env::var("OPEN_DART_API_KEY")
                    .expect("OPEN_DART_API_KEY must be set as an environment variable"),
                name: name.to_string(),
                age,
            }
        );
    }

    #[test]
    fn json_body_should_work_with_all_fields_set() {
        json_body!(
            ResponseBody {
                pub name: String,
                pub age: u16,
            }
        );

        let status = "success";
        let message = "OK";
        let name = "John Doe";
        let age = 42;
        let body = ResponseBody {
            status: status.to_string(),
            message: message.to_string(),
            name: name.to_string(),
            age,
        };

        assert_eq!(body.status, status);
        assert_eq!(body.message, message);
        assert_eq!(body.name, name);
        assert_eq!(body.age, age);
    }
}
