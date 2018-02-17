extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use syn::{punctuated, Data, DeriveInput, Fields, Generics, PredicateType, TypeGenerics,
          TypeParamBound, WhereClause, WherePredicate};
use quote::ToTokens;

#[proc_macro_derive(Show)]
pub fn derive_show(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = input.ident;
    let body = body(&input.data);
    let (impl_generics, ty_generics, where_clause) = generics(&input.generics);

    let expanded = quote! {
        impl #impl_generics ::moho::renderer::Show<R> for #name #ty_generics #where_clause {
            fn show(&self, renderer: &mut R) -> ::moho::Result<()> {
                #body
            }
        }
    };
    expanded.into()
}

fn generics(generics: &Generics) -> (quote::Tokens, TypeGenerics, quote::Tokens) {
    let param: syn::GenericParam = parse_quote!(R: ::moho::renderer::Renderer);
    let mut gen_clone = generics.clone();
    gen_clone.params.push(param);
    let impl_generics = gen_clone.split_for_impl().0;
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let mut where_clause = where_clause.cloned().unwrap_or_else(|| WhereClause {
        predicates: punctuated::Punctuated::new(),
        where_token: Default::default(),
    });
    where_clause
        .predicates
        .push(WherePredicate::Type(PredicateType {
            lifetimes: None,
            colon_token: Default::default(),
            bounded_ty: parse_quote!(T),
            bounds: Some(punctuated::Pair::End(TypeParamBound::Trait(parse_quote!(
                ::moho::renderer::Draw<R>
            )))).into_iter()
                .collect(),
        }));
    (
        impl_generics.into_tokens(),
        ty_generics,
        where_clause.into_tokens(),
    )
}

fn body(data: &Data) -> quote::Tokens {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let names = fields.named.iter().map(|f| f.ident);
                quote! {
                    #(renderer.show(&self.#names)?;)*
                    Ok(())
                }
            }
            Fields::Unnamed(_) => unimplemented!("Struct::Unnamed"),
            Fields::Unit => unimplemented!("Struct::Unit"),
        },
        Data::Enum(_) => unimplemented!("Enum"),
        Data::Union(_) => unimplemented!("Union"),
    }
}
