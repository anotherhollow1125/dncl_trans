use std::fs;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    Token,
};
use syn::{Ident, LitInt, LitStr};

pub struct MacroInput {
    pub model: Option<String>,
    pub seed: Option<u64>,
    pub max_completion_tokens: Option<u64>,
    pub dncl_code: TokenStream,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut model: Option<String> = None;
        let mut seed = None;
        let mut max_completion_tokens = None;
        let mut file_content = None;

        fn parse_puncts(input: ParseStream) -> syn::Result<()> {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
            }
            Ok(())
        }

        while input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            let ident = input.parse::<Ident>()?;
            input.parse::<syn::Token![=]>()?;
            match ident {
                i if i == "model" => {
                    let value = input.parse::<LitStr>()?;
                    model = Some(value.value());
                }
                i if i == "max_completion_tokens" => {
                    let value = input.parse::<LitInt>()?;
                    max_completion_tokens = Some(value.base10_parse()?);
                }
                i if i == "seed" => {
                    let value = input.parse::<LitInt>()?;
                    seed = Some(value.base10_parse()?);
                }
                i if i == "file" => {
                    let value = input.parse::<LitStr>()?;
                    let file_path = value.value();

                    file_content = Some(fs::read_to_string(file_path).into_syn(value.span())?);
                }
                _ => return Err(syn::Error::new(ident.span(), "unexpected field")),
            }
            parse_puncts(input)?;
        }

        let dncl_code: TokenStream = if let Some(file_content) = file_content {
            if file_content.is_empty() {
                return Err(syn::Error::new(Span::call_site(), "file is empty"));
            }

            LitStr::new(&file_content, Span::call_site()).into_token_stream()
        } else if input.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            let value = lit.value().replace("\n", ";");

            if value.is_empty() {
                return Err(syn::Error::new(lit.span(), "code is empty"));
            }

            LitStr::new(&value, lit.span()).into_token_stream()
        } else {
            if input.is_empty() {
                return Err(syn::Error::new(Span::call_site(), "code is empty"));
            }

            input.parse::<TokenStream>()?
        };

        Ok(Self {
            model,
            seed,
            max_completion_tokens,
            dncl_code,
        })
    }
}

/// `Result<T, E>` -> `syn::Result<T>` に変換するトレイト
///
/// `res.into_syn(span)?;` のような使い方を想定
pub trait IntoSynRes<T> {
    fn into_syn(self, span: Span) -> syn::Result<T>;
}

impl<T, E> IntoSynRes<T> for Result<T, E>
where
    E: std::fmt::Display + std::fmt::Debug,
{
    fn into_syn(self, span: Span) -> syn::Result<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(syn::Error::new(span, err)),
        }
    }
}
