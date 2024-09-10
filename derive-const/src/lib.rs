#[proc_macro_derive(Const)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro2::TokenStream::new().into()
}
