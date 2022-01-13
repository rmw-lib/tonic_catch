mod parse;

use quote::quote;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use crate::parse::Item;

#[proc_macro_attribute]
pub fn tonic_catch(_: TokenStream, input: TokenStream) -> TokenStream {
  let mut item = parse_macro_input!(input as Item);

  println!("{:?}", input);
  input
}
