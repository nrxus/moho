use super::{Circle, Intersect, Line, Shape};

use glm;

pub struct Rectangle {
    pub dims: glm::DVec2,
    pub center: glm::DVec2,
}

impl Rectangle {
    pub fn get_lines(&self) -> [Line; 4] {
        let tl = self.center + glm::dvec2(-self.dims.x / 2., -self.dims.y / 2.);
        let bl = self.center + glm::dvec2(-self.dims.x / 2., self.dims.y / 2.);
        let br = self.center + glm::dvec2(self.dims.x / 2., self.dims.y / 2.);
        let tr = self.center + glm::dvec2(self.dims.x / 2., -self.dims.y / 2.);

        [(tl, bl), (tl, tr), (bl, br), (br, tr)]
    }
}

impl Shape for Rectangle {
    fn get_center(&self) -> glm::DVec2 {
        self.center
    }

    fn contains(&self, point: &glm::DVec2) -> bool {
        let half_x = self.dims.x / 2.;
        let half_y = self.dims.y / 2.;

        !(self.center.x - half_x > point.x) && !(self.center.x + half_x < point.x) &&
        !(self.center.y - half_y > point.y) && !(self.center.y + half_y < point.y)
    }
}

impl Intersect<Rectangle> for Rectangle {
    fn intersects(&self, other: &Rectangle) -> bool {
        let half_x = self.dims.x / 2.;
        let half_y = self.dims.y / 2.;

        !(self.center.x - half_x > other.center.x + other.dims.x / 2.) &&
        !(self.center.x + half_x < other.center.x - other.dims.x / 2.) &&
        !(self.center.y - half_y > other.center.y + other.dims.y / 2.) &&
        !(self.center.y + half_y < other.center.y - other.dims.y / 2.)
    }
}

impl Intersect<Circle> for Rectangle {
    fn intersects(&self, other: &Circle) -> bool {
        other.intersects(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rectangle_no_contains() {
        let rectangle = Rectangle {
            dims: glm::dvec2(2_f64, 3_f64),
            center: glm::dvec2(3_f64, 0_f64),
        };
        let point = glm::dvec2(3.5_f64, -1_f64);
        assert!(rectangle.contains(&point));
    }

    #[test]
    fn rectangle_contains() {
        let rectangle = Rectangle {
            dims: glm::dvec2(2_f64, 3_f64),
            center: glm::dvec2(3_f64, 0_f64),
        };
        let point = glm::dvec2(3.5_f64, 2_f64);
        assert!(!rectangle.contains(&point));
    }

    #[test]
    fn rectangle_rectangle_no_intersect() {
        let rectangle_a = Rectangle {
            dims: glm::dvec2(2_f64, 3_f64),
            center: glm::dvec2(3_f64, 0_f64),
        };

        let rectangle_b = Rectangle {
            dims: glm::dvec2(5_f64, 2_f64),
            center: glm::dvec2(0_f64, 8_f64),
        };
        assert!(!rectangle_a.intersects(&rectangle_b));
        assert!(!rectangle_b.intersects(&rectangle_a));
    }

    #[test]
    fn rectangle_rectangle_intersect() {
        let rectangle_a = Rectangle {
            dims: glm::dvec2(2_f64, 3_f64),
            center: glm::dvec2(3_f64, 0_f64),
        };

        let rectangle_b = Rectangle {
            dims: glm::dvec2(1_f64, 2_f64),
            center: glm::dvec2(2.5_f64, 1_f64),
        };
        assert!(rectangle_a.intersects(&rectangle_b));
        assert!(rectangle_b.intersects(&rectangle_a));
    }

    #[test]
    fn rectangle_circle_no_intersect() {
        let rectangle = Rectangle {
            dims: glm::dvec2(5_f64, 2_f64),
            center: glm::dvec2(6_f64, 0_f64),
        };

        let circle = Circle {
            radius: 3_f64,
            center: glm::dvec2(1_f64, 3_f64),
        };

        assert!(!rectangle.intersects(&circle));
    }

    #[test]
    fn rectangle_inside_circle() {
        let rectangle = Rectangle {
            dims: glm::dvec2(1_f64, 2_f64),
            center: glm::dvec2(3_f64, 5_f64),
        };

        let circle = Circle {
            radius: 5_f64,
            center: glm::dvec2(2_f64, 3_f64),
        };

        assert!(rectangle.intersects(&circle));
    }

    #[test]
    fn circle_inside_rectangle() {
        let rectangle = Rectangle {
            dims: glm::dvec2(5_f64, 7_f64),
            center: glm::dvec2(4_f64, 2_f64),
        };

        let circle = Circle {
            radius: 1_f64,
            center: glm::dvec2(5_f64, 3_f64),
        };

        assert!(rectangle.intersects(&circle));
    }

    #[test]
    fn rectangle_circle_intersect() {
        let rectangle = Rectangle {
            dims: glm::dvec2(2_f64, 2_f64),
            center: glm::dvec2(5_f64, 3_f64),
        };

        let circle = Circle {
            radius: 2_f64,
            center: glm::dvec2(2_f64, 3_f64),
        };

        assert!(rectangle.intersects(&circle));
    }
}
