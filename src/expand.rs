use crate::Item;
use syn::ImplItem;

pub fn expand(input: &mut Item) {
  match input {
    Item::Impl(input) => {
      for inner in &mut input.items {
        if let ImplItem::Method(method) = inner {
          let sig = &mut method.sig;
          if sig.asyncness.is_some() {}
        }
      }
    }
  }
}
