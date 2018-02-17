extern crate moho;
#[macro_use]
extern crate moho_derive;
#[macro_use]
extern crate synstructure;

use moho::texture::Image;

mod foo {
    use moho::texture::Image;
    use std::rc::Rc;

    #[derive(Show)]
    struct Assets<T> {
        image: Image<T>,
    }
}
