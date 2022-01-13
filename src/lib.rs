use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn tonic_catch(_: TokenStream, input: TokenStream) -> TokenStream {
  println!("{:?}", input);
  input
}
