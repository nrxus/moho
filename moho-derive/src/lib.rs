extern crate proc_macro;
extern crate quote;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate synstructure;

decl_derive!([Show] => show_derive);

fn show_derive(mut structure: synstructure::Structure) -> quote::Tokens {
    let body = structure.each(|bi| {
        quote! {
            renderer.show(#bi)?;
        }
    });
    structure
        .add_impl_generic(parse_quote!(R: ::moho::renderer::Renderer))
        .bounds(synstructure::BoundsToAdd::Fields)
        .bound_impl(
            quote!(::moho::renderer::Show<R>),
            quote!{
                fn show(&self, renderer: &mut R) -> ::moho::Result<()> {
                    match *self { #body }
                    Ok(())
                }
            },
        )
}
