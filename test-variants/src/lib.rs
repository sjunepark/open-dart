use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Data, DeriveInput, Ident, Result};

struct MacroArgs {
    name: Ident,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        Ok(MacroArgs { name })
    }
}

#[proc_macro_attribute]
pub fn test_variants(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as MacroArgs);
    let input = parse_macro_input!(item as DeriveInput);
    let enum_name = &input.ident;
    let struct_name = &args.name;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("This macro only works on enums"),
    };

    let variant_idents: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    let expanded = quote! {
        #input

         impl #struct_name {
            #(
                pub const #variant_idents: Self = Self(#enum_name::#variant_idents);
            )*
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn serialize() {
                #(
                    assert_eq!(
                        serde_json::to_string(&#struct_name::#variant_idents).expect("Failed to serialize"),
                        concat!(r#"""#, stringify!(#variant_idents), r#"""#)
                    );
                )*
            }

            #[test]
            fn deserialize() {
                #(
                    assert_eq!(
                        serde_json::from_str::<#struct_name>(concat!(r#"""#, stringify!(#variant_idents), r#"""#))
                            .expect("Failed to deserialize"),
                        #struct_name::#variant_idents
                    );
                )*
            }

            #[test]
            fn display() {
                #(
                    assert_eq!(#struct_name::#variant_idents.to_string(), stringify!(#variant_idents));
                )*
            }
        }
    };

    TokenStream::from(expanded)
}
