pub mod circle;
pub mod rectangle;

pub use self::{circle::Circle, rectangle::Rectangle};

pub type Line = (glm::DVec2, glm::DVec2);

pub trait Intersect<S> {
    fn intersects(&self, other: &S) -> bool;
    fn mtv(&self, other: &S) -> Option<glm::DVec2>;
}

pub trait Shape {
    fn center(&self) -> glm::DVec2;
    fn contains(&self, point: &glm::DVec2) -> bool;
    fn nudge(self, nudge: glm::DVec2) -> Self;
    fn center_at(self, center: glm::DVec2) -> Self;

    fn distance(&self, other: &impl Shape) -> f64 {
        glm::distance(self.center(), other.center())
    }
}

pub struct Axis(glm::DVec2);

impl Axis {
    fn mtv(&self, a: &[glm::DVec2], b: &[glm::DVec2]) -> Option<glm::DVec2> {
        let a = self.project(a);
        let b = self.project(b);
        self.mtv_range(a, b)
    }

    fn mtv_circle(&self, a: &[glm::DVec2], circle: &Circle) -> Option<glm::DVec2> {
        let &Axis(axis) = self;
        let a = self.project(a);
        let center = glm::dot(axis, circle.center);
        let b = (center - circle.radius, center + circle.radius);
        self.mtv_range(a, b)
    }

    fn mtv_range(&self, a: (f64, f64), b: (f64, f64)) -> Option<glm::DVec2> {
        let double_mag = (a.1 - a.0) + (b.1 - b.0) - ((a.1 + a.0) - (b.1 + b.0)).abs();
        if double_mag <= 0. {
            None
        } else {
            let &Axis(axis) = self;
            let mag = double_mag / 2.;
            Some(axis * mag)
        }
    }

    fn project(&self, verts: &[glm::DVec2]) -> (f64, f64) {
        let &Axis(axis) = self;
        let min = verts
            .iter()
            .map(|&v| glm::dot(axis, v))
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();

        let max = verts
            .iter()
            .map(|v| glm::dot(axis, *v))
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();

        (min, max)
    }
}

trait FindMtv {
    fn find_mtv(self, object: &impl Shape, fixed: &impl Shape) -> Option<glm::DVec2>;
}

impl<I> FindMtv for I
where
    I: Iterator<Item = Option<glm::DVec2>>,
{
    fn find_mtv(self, object: &impl Shape, fixed: &impl Shape) -> Option<glm::DVec2> {
        let mtvs: Vec<_> = self.collect::<Option<_>>()?;
        let min = *mtvs
            .iter()
            .min_by(|&&x, &&y| glm::length(x).partial_cmp(&glm::length(y)).unwrap())?;
        let reversed = glm::dot(object.center() - fixed.center(), min) < 0.;
        Some(if reversed { min * -1. } else { min })
    }
}
