use proc_macro::TokenStream;

mod impls;

/// DNCLプログラムをRustプログラムにトランスパイルするマクロ
///
/// マクロの使用例:
/// ```rust
/// dncl_trans::dncl!(
///     @model = "o1-preview";
///     @max_completion_tokens = 4096;
///     @seed = 123456;
///     @editing = false; // 編集中はtrueにすることでAPIを叩きに行かないようにする
///     // @file = "もしファイル分割しているならこの変数で指定.dncl";
///
///     r#"
///     /* ここにDNCL記法のコードを書く */
///     "#
/// );
/// ```
#[proc_macro]
pub fn dncl(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as impls::MacroInput);

    impls::dncl_impl(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
