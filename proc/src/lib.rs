mod expand;
mod parse;

use crate::{expand::expand, parse::Item};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn tonic_catch(_: TokenStream, input: TokenStream) -> TokenStream {
  let mut item = parse_macro_input!(input as Item);
  expand(&mut item);
  TokenStream::from(quote!(
    #[tonic::async_trait]
    #item
  ))
}
