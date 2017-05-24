use super::{Axis, Circle, Intersect, Line, Shape};

use glm;

pub struct Rectangle {
    pub dims: glm::DVec2,
    pub top_left: glm::DVec2,
}

impl Rectangle {
    pub fn get_lines(&self) -> [Line; 4] {
        let tl = self.top_left;
        let bl = self.top_left + glm::dvec2(0., self.dims.y / 2.);
        let tr = self.top_left + glm::dvec2(self.dims.x, 0.);
        let br = self.top_left + self.dims;

        [(tl, bl), (tl, tr), (bl, br), (br, tr)]
    }

    pub fn verts(&self) -> [glm::DVec2; 4] {
        [self.top_left,
         self.top_left + glm::dvec2(self.dims.x, 0.),
         self.top_left + self.dims,
         self.top_left + glm::dvec2(0., self.dims.y)]
    }

    pub fn axes(&self) -> [Axis; 2] {
        [Axis(glm::dvec2(0., 1.)), Axis(glm::dvec2(1., 0.))]
    }
}

impl Shape for Rectangle {
    fn center(&self) -> glm::DVec2 {
        self.top_left + self.dims / 2.
    }

    fn contains(&self, point: &glm::DVec2) -> bool {
        !(self.top_left.x > point.x) && !(self.top_left.x + self.dims.x < point.x) &&
        !(self.top_left.y > point.y) && !(self.top_left.y + self.dims.y < point.y)
    }
}

impl Intersect<Rectangle> for Rectangle {
    fn intersects(&self, other: &Rectangle) -> bool {
        !(self.top_left.x > other.top_left.x + other.dims.x) &&
        !(self.top_left.x + self.dims.x < other.top_left.x) &&
        !(self.top_left.y > other.top_left.y + other.dims.y) &&
        !(self.top_left.y + self.dims.y < other.top_left.y)
    }

    fn mtv(&self, fixed: &Rectangle) -> Option<glm::DVec2> {
        let my_verts = self.verts();
        let fixed_verts = fixed.verts();

        self.axes()
            .iter()
            .map(|a| a.mtv(&my_verts, &fixed_verts))
            .collect::<Option<Vec<_>>>()
            .map(|p| super::pick_min(&p))
            .map(|v| {
                     let p = glm::dot(self.center() - fixed.center(), v);
                     if p < 0. { v * -1. } else { v }
                 })
    }
}

impl Intersect<Circle> for Rectangle {
    fn intersects(&self, other: &Circle) -> bool {
        other.intersects(self)
    }

    fn mtv(&self, fixed: &Circle) -> Option<glm::DVec2> {
        fixed.mtv(self).map(|v| v * -1.)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rectangle_no_contains() {
        let rectangle = Rectangle {
            dims: glm::dvec2(2_f64, 3_f64),
            top_left: glm::dvec2(2_f64, -1.5),
        };
        let point = glm::dvec2(3.5_f64, -1_f64);
        assert!(rectangle.contains(&point));
    }

    #[test]
    fn rectangle_contains() {
        let rectangle = Rectangle {
            dims: glm::dvec2(2_f64, 3_f64),
            top_left: glm::dvec2(2_f64, -1.5),
        };
        let point = glm::dvec2(3.5_f64, 2_f64);
        assert!(!rectangle.contains(&point));
    }

    #[test]
    fn rectangle_rectangle_no_intersect() {
        let rectangle_a = Rectangle {
            dims: glm::dvec2(2_f64, 3_f64),
            top_left: glm::dvec2(2_f64, -1.5),
        };

        let rectangle_b = Rectangle {
            dims: glm::dvec2(5_f64, 2_f64),
            top_left: glm::dvec2(-2.5, 7_f64),
        };
        assert!(!rectangle_a.intersects(&rectangle_b));
        assert_eq!(rectangle_a.mtv(&rectangle_b), None);
        assert!(!rectangle_b.intersects(&rectangle_a));
        assert_eq!(rectangle_b.mtv(&rectangle_a), None);
    }

    #[test]
    fn rectangle_rectangle_intersect() {
        let rectangle_a = Rectangle {
            dims: glm::dvec2(2_f64, 3_f64),
            top_left: glm::dvec2(2_f64, -1.5),
        };

        let rectangle_b = Rectangle {
            dims: glm::dvec2(1_f64, 2_f64),
            top_left: glm::dvec2(2_f64, 0_f64),
        };
        assert!(rectangle_a.intersects(&rectangle_b));
        assert_eq!(rectangle_a.mtv(&rectangle_b), Some(glm::dvec2(1., 0.)));
        assert!(rectangle_b.intersects(&rectangle_a));
        assert_eq!(rectangle_b.mtv(&rectangle_a), Some(glm::dvec2(-1., 0.)));
    }

    #[test]
    fn rectangle_circle_no_intersect() {
        let rectangle = Rectangle {
            dims: glm::dvec2(5_f64, 2_f64),
            top_left: glm::dvec2(3.5, -1.),
        };

        let circle = Circle {
            radius: 3_f64,
            center: glm::dvec2(1_f64, 3_f64),
        };

        assert!(!rectangle.intersects(&circle));
        assert_eq!(rectangle.mtv(&circle), None);
    }

    #[test]
    fn rectangle_inside_circle() {
        let rectangle = Rectangle {
            dims: glm::dvec2(1_f64, 2_f64),
            top_left: glm::dvec2(2.5, 4_f64),
        };

        let circle = Circle {
            radius: 5_f64,
            center: glm::dvec2(2.5, 3_f64),
        };

        assert!(rectangle.intersects(&circle));
        assert_eq!(rectangle.mtv(&circle), Some(glm::dvec2(0., 4.)));
    }

    #[test]
    fn circle_inside_rectangle() {
        let rectangle = Rectangle {
            dims: glm::dvec2(5_f64, 7_f64),
            top_left: glm::dvec2(1.5, -1.5),
        };

        let circle = Circle {
            radius: 1_f64,
            center: glm::dvec2(5_f64, 3_f64),
        };

        assert!(rectangle.intersects(&circle));
        assert_eq!(rectangle.mtv(&circle), Some(glm::dvec2(-2.5, 0.)));
    }

    #[test]
    fn rectangle_circle_intersect() {
        let rectangle = Rectangle {
            dims: glm::dvec2(2., 2.),
            top_left: glm::dvec2(4., 2.),
        };

        let circle = Circle {
            radius: 2.1,
            center: glm::dvec2(2., 3.),
        };

        assert!(rectangle.intersects(&circle));
        let mtv = rectangle.mtv(&circle);
        assert!(mtv.is_some());
        let mtv = mtv.unwrap();
        assert!(glm::length(mtv - glm::dvec2(0.1, 0.)) < 0.00001);
    }
}
