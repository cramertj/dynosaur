#![doc = include_str!("../README.md")]

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, ItemTrait, Result, Signature, Token, TraitItem, TraitItemFn,
};

mod util;

struct Attrs;

impl Parse for Attrs {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![dyn]>()?;
        Ok(Self)
    }
}

/// Given a trait like
///
/// ```
/// #[dynosaur::make_dyn(dyn)]
/// trait MyTrait {
///     type Item;
///     async fn foo(&self) -> Self::Item;
/// }
/// ```
///
/// The above example causes the trait to be rewritten as:
///
/// ```
/// trait ErasedMyTrait {
///     type Item;
///     fn foo<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Self::Item> + 'a>>;
/// }
/// ```
#[proc_macro_attribute]
pub fn make_dyn(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let _attrs = parse_macro_input!(attr as Attrs);
    let item = parse_macro_input!(item as ItemTrait);

    let erased_trait = mk_erased_trait(&item);

    quote! {
        #item

        #erased_trait
    }
    .into()
}

fn mk_erased_trait(item: &ItemTrait) -> TokenStream {
    let erased_trait = ItemTrait {
        ident: Ident::new(&format!("Erased{}", item.ident), item.ident.span()),
        items: item
            .items
            .iter()
            .map(|item| {
                if let TraitItem::Fn(
                    trait_item_fn @ TraitItemFn {
                        sig:
                            Signature {
                                asyncness: Some(..),
                                ..
                            },
                        ..
                    },
                ) = item
                {
                    TraitItem::Fn(TraitItemFn {
                        sig: Signature {
                            asyncness: None,
                            ..trait_item_fn.sig.clone()
                        },
                        ..trait_item_fn.clone()
                    })
                } else {
                    item.clone()
                }
            })
            .collect(),
        ..item.clone()
    };
    quote! { #erased_trait }
}
