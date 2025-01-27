use proc_macro::TokenStream;

mod impls;

#[proc_macro]
pub fn dncl(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as impls::MacroInput);

    impls::dncl_impl(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
