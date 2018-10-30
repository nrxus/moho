extern crate proc_macro2;
extern crate quote;
extern crate syn;
extern crate synstructure;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use synstructure::{decl_derive, AddBounds, Structure};

decl_derive!([Show, attributes(moho)] => show_derive);

fn show_derive(mut structure: Structure) -> TokenStream {
    let moho_skip = parse_quote!(#[moho(skip)]);
    structure.filter(|bi| !bi.ast().attrs.iter().any(|a| *a == moho_skip));
    let body = structure.each(|bi| {
        quote! {
            renderer.show(#bi)?;
        }
    });

    structure.add_bounds(AddBounds::Fields).gen_impl(quote! {
        gen impl<R: moho::renderer::Renderer> moho::renderer::Show<R> for @Self {
            fn show(&self, renderer: &mut R) -> moho::Result<()> {
                match *self { #body }
                Ok(())
            }
        }
    })
}
