use proc_macro2::{Group, Span, TokenStream, TokenTree};
use std::iter::FromIterator;
use syn::visit_mut::{self, VisitMut};
use syn::{
    ImplItem,
};
use crate::Item;

pub fn expand(input: &mut Item) {
  match input {
    Item::Impl(input) => {
      for inner in &mut input.items {
        if let ImplItem::Method(method) = inner {
          let sig = &mut method.sig;
          if sig.asyncness.is_some() {

          }
        }
      }
    }
  }
}
