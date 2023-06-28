use proc_macro::{TokenStream, TokenTree};
use quote::quote;

extern crate proc_macro;

#[proc_macro]
pub fn concat_idents(ts: TokenStream) -> TokenStream {
  let mut buf = String::new();
  for t in ts {
    let TokenTree::Ident(ident) = t else {
      let t = t.to_string();
      return quote! {
        compile_error!(concat!("Ident expected. Found: '", #t, "'"));
      }.into();
    };
    buf += &ident.to_string();
  }
  quote!(#buf).into()
}
