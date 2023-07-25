use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Ident;
use quote::quote;
use syn::{FnArg, PatType, Pat, punctuated::Punctuated, token::Comma, Result};

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


#[proc_macro_attribute]
pub fn unwind_catch(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  match unwind_catch_impl(attr.into(), item.into()) {
    Ok(res) => res.into(),
    Err(err) => err.into_compile_error().into(),
  }
}

fn unwind_catch_impl(
  attr: proc_macro2::TokenStream,
  item: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
  let attr = syn::parse2::<syn::Expr>(attr)?;
  let mut original_fn: syn::ItemFn = syn::parse2(item)?;
  let syn::Signature {
    abi,
    inputs,
    output,
    generics,
    ident: original_ident,
    unsafety,
    fn_token,
    ..
  } = original_fn.sig.clone();
  let mut inner_ident = quote::format_ident!("{original_ident}_inner");
  inner_ident.set_span(original_ident.span());
  original_fn.sig.ident = inner_ident.clone();
  let inputs_bound = inputs.clone().into_iter().map(|arg| {
    let FnArg::Typed(PatType{pat, ..}) = arg else {
      return Err(syn::Error::new_spanned(arg, "Unxepected receiver arg"));
    };
    let Pat::Ident(ident) = pat.as_ref() else {
      return Err(syn::Error::new_spanned(pat, "Unexpected non ident pattern in function signature"));
    };
    Ok(ident.ident.clone())
  }).collect::<Result<Punctuated<Ident, Comma>>>()?;
  Ok(quote! {
    #unsafety #abi #fn_token #original_ident #generics(#inputs) #output {
      #original_fn
      match ::std::panic::catch_unwind(||unsafe {#inner_ident(#inputs_bound)}) {
        Ok(res) => res,
        Err(_) => #attr,
      }
    }
  })
}

#[proc_macro_attribute]
pub fn unwind_handle(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  match unwind_handle_impl(attr.into(), item.into()) {
    Ok(res) => res.into(),
    Err(err) => err.into_compile_error().into(),
  }
}

fn unwind_handle_impl(
  attr: proc_macro2::TokenStream,
  item: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
  let attr = syn::parse2::<syn::Ident>(attr)?;
  let mut original_fn: syn::ItemFn = syn::parse2(item)?;
  let syn::Signature {
    abi,
    inputs,
    output,
    generics,
    ident: original_ident,
    unsafety,
    fn_token,
    ..
  } = original_fn.sig.clone();
  let mut inner_ident = quote::format_ident!("{original_ident}_inner");
  inner_ident.set_span(original_ident.span());
  original_fn.sig.ident = inner_ident.clone();
  let inputs_bound = inputs.clone().into_iter().map(|arg| {
    let FnArg::Typed(PatType{pat, ..}) = arg else {
      return Err(syn::Error::new_spanned(arg, "Unxepected receiver arg"));
    };
    let Pat::Ident(ident) = pat.as_ref() else {
      return Err(syn::Error::new_spanned(pat, "Unexpected non ident pattern in function signature"));
    };
    Ok(ident.ident.clone())
  }).collect::<Result<Punctuated<Ident, Comma>>>()?;
  Ok(quote! {
    #unsafety #abi #fn_token #original_ident #generics(#inputs) #output {
      #original_fn
      match ::std::panic::catch_unwind(||unsafe {#inner_ident(#inputs_bound)}) {
        Ok(res) => res,
        Err(err) => #attr(err),
      }
    }
  })
}
