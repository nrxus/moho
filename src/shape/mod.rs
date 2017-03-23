pub mod circle;
pub mod rectangle;

pub use self::circle::Circle;
pub use self::rectangle::Rectangle;

use glm;

pub type Line = (glm::DVec2, glm::DVec2);

pub trait Intersect<S> {
    fn intersects(&self, other: &S) -> bool;
}

pub trait Shape {
    fn get_center(&self) -> glm::DVec2;
    fn contains(&self, point: &glm::DVec2) -> bool;
    fn distance<S: Shape>(&self, other: &S) -> f64 {
        glm::distance(self.get_center(), other.get_center())
    }
}
