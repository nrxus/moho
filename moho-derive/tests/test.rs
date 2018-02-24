extern crate moho;
#[macro_use]
extern crate moho_derive;

mod foo {
    use moho::texture::Image;
    use std::rc::Rc;

    #[derive(Show)]
    struct Assets<T> {
        image: Image<T>,
        other: Rc<T>,
    }
}
