use crate::Item;
use quote::quote;
use syn::{ImplItem, parse_quote};

pub fn expand(input: &mut Item) {
  match input {
    Item::Impl(input) => {
      for inner in &mut input.items {
        if let ImplItem::Method(method) = inner {
          let block = &mut method.block;
          let stmts = &block.stmts;

          let q = quote!(
            Ok((async { #(#stmts)* }.await as std::result::Result<_,Error>)?)
          );
          block.stmts = parse_quote!(#q);
        }
      }
    }
  }
}
