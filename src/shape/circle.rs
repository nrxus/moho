use super::{Intersect, Line, Shape, Rectangle};

use glm;

#[derive(Clone, Copy)]
pub struct Circle {
    pub radius: f64,
    pub center: glm::DVec2,
}

impl Shape for Circle {
    fn get_center(&self) -> glm::DVec2 {
        self.center
    }

    fn contains(&self, point: &glm::DVec2) -> bool {
        let distance = glm::distance(self.center, *point);
        distance < self.radius
    }
}

impl Intersect<Rectangle> for Circle {
    fn intersects(&self, other: &Rectangle) -> bool {
        self.contains(&other.get_center()) || other.contains(&self.center) ||
        other.get_lines().iter().any(|l| self.intersects(l))
    }
}

impl Intersect<Circle> for Circle {
    fn intersects(&self, other: &Circle) -> bool {
        let distance = glm::distance(self.center, other.center);
        distance < (self.radius + other.radius)
    }
}

impl Intersect<Line> for Circle {
    fn intersects(&self, other: &Line) -> bool {
        let length = other.1 - other.0;
        let dist_center = other.0 - self.center;
        let len_sq = glm::dot(length, length);
        let b = 2_f64 * glm::dot(dist_center, length);
        let c = glm::dot(dist_center, dist_center) - self.radius * self.radius;
        let mut discriminant = b * b - 4_f64 * len_sq * c;

        if discriminant < 0_f64 {
            return false;
        }

        discriminant = discriminant.sqrt();

        let t1 = (-b - discriminant) / (2_f64 * len_sq);
        let t2 = (-b + discriminant) / (2_f64 * len_sq);

        t1 >= 0_f64 && t1 <= 1_f64 || t2 >= 0_f64 && t2 <= 1_f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn circle_no_contains() {
        let circle = Circle {
            radius: 1_f64,
            center: glm::dvec2(2_f64, 0_f64),
        };
        let point = glm::dvec2(4_f64, 4_f64);
        assert!(!circle.contains(&point));
    }

    #[test]
    fn circle_contains() {
        let circle = Circle {
            radius: 3_f64,
            center: glm::dvec2(2_f64, 3_f64),
        };
        let point = glm::dvec2(4_f64, 4_f64);
        assert!(circle.contains(&point));
    }

    #[test]
    fn circle_circle_no_intersect() {
        let circle_a = Circle {
            radius: 3_f64,
            center: glm::dvec2(2_f64, 3_f64),
        };

        let circle_b = Circle {
            radius: 1_f64,
            center: glm::dvec2(4_f64, 7_f64),
        };

        assert!(!circle_a.intersects(&circle_b));
        assert!(!circle_b.intersects(&circle_a));
    }

    #[test]
    fn circle_circle_intersect() {
        let circle_a = Circle {
            radius: 3_f64,
            center: glm::dvec2(2_f64, 3_f64),
        };

        let circle_b = Circle {
            radius: 1_f64,
            center: glm::dvec2(1_f64, 5_f64),
        };

        assert!(circle_a.intersects(&circle_b));
        assert!(circle_b.intersects(&circle_a));
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

        assert!(!circle.intersects(&rectangle));
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

        assert!(circle.intersects(&rectangle));
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

        assert!(circle.intersects(&rectangle));
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

        assert!(circle.intersects(&rectangle));
    }
}
