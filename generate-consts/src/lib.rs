use proc_macro::TokenStream;
use quote::{format_ident, quote};
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
pub fn generate_consts(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as MacroArgs);
    let input = parse_macro_input!(item as DeriveInput);
    let enum_name = &input.ident;
    let struct_name = &args.name;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("This macro only works on enums"),
    };

    let constants = variants.iter().map(|v| {
        let ident = &v.ident;
        let upper_ident = format_ident!("{}", ident.to_string().to_uppercase());
        let docs = v
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .collect::<Vec<_>>();

        quote! {
            #(#docs)*
            pub const #upper_ident: Self = Self(#enum_name::#ident);
        }
    });

    let expanded = quote! {
        #input

        impl #struct_name {
            #(#constants)*
        }
    };

    TokenStream::from(expanded)
}
