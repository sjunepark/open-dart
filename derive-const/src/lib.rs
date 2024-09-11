use syn::{parse_macro_input, DeriveInput, ItemEnum, Meta};

#[proc_macro_attribute]
pub fn impl_const(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as Meta);
    let input = parse_macro_input!(input as DeriveInput);
    inner(attr, input).into()
}

fn inner(attr: Meta, input: DeriveInput) -> proc_macro2::TokenStream {
    println!("{:#?}", attr);
    println!("{:#?}", input);

    let enum_ident = syn::Ident::new("Inner", proc_macro2::Span::call_site());
    let enum_definition = find_enum_definition(&enum_ident);

    eprintln!("{:#?}", enum_definition);

    // todo
    proc_macro2::TokenStream::new()
}

fn find_enum_definition(ident: &syn::Ident) -> ItemEnum {
    let input_source = proc_macro2::Span::call_site()
        .source_text()
        .expect("no source text");
    let syntax_tree = syn::parse_file(&input_source).expect("failed to parse file");

    syntax_tree
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Enum(item) if item.ident == *ident => Some(item.clone()),
            _ => None,
        })
        .next()
        .expect("no enum definition found")
}
