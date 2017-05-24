use super::{Axis, Intersect, Line, Shape, Rectangle};

use glm;

#[derive(Clone, Copy)]
pub struct Circle {
    pub radius: f64,
    pub center: glm::DVec2,
}

impl Shape for Circle {
    fn center(&self) -> glm::DVec2 {
        self.center
    }

    fn contains(&self, point: &glm::DVec2) -> bool {
        let distance = glm::distance(self.center, *point);
        distance < self.radius
    }
}

impl Intersect<Rectangle> for Circle {
    fn intersects(&self, other: &Rectangle) -> bool {
        self.contains(&other.top_left) || other.contains(&self.center) ||
        other.get_lines().iter().any(|l| self.intersects(l))
    }

    fn mtv(&self, fixed: &Rectangle) -> Option<glm::DVec2> {
        let verts = fixed.verts();
        let closest = verts
            .iter()
            .map(|&v| v - self.center)
            .min_by(|&x, &y| glm::dot(x, x).partial_cmp(&glm::dot(y, y)).unwrap())
            .unwrap()
            .clone();
        let circle_axis = Axis(glm::normalize(closest));
        fixed
            .axes()
            .iter()
            .chain([circle_axis].iter())
            .map(|a| a.mtv_circle(&verts, self))
            .collect::<Option<Vec<_>>>()
            .map(|p| super::pick_min(&p))
            .map(|v| {
                     let p = glm::dot(self.center() - fixed.center(), v);
                     if p < 0. { v * -1. } else { v }
                 })
    }
}

impl Intersect<Circle> for Circle {
    fn intersects(&self, other: &Circle) -> bool {
        let distance = self.distance(other);
        distance < (self.radius + other.radius)
    }

    fn mtv(&self, fixed: &Circle) -> Option<glm::DVec2> {
        let distance = self.center - fixed.center;
        let len = glm::length(distance);
        let min_gap = self.radius + fixed.radius;
        if len < min_gap {
            Some(glm::ext::normalize_to(distance, min_gap - len))
        } else {
            None
        }
    }
}

impl Intersect<Line> for Circle {
    fn intersects(&self, other: &Line) -> bool {
        let line_center = (other.1 + other.0) / 2.;
        if self.contains(&line_center) {
            return true;
        }
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

        t1 > 0_f64 && t1 < 1_f64 || t2 > 0_f64 && t2 < 1_f64
    }

    fn mtv(&self, fixed: &Line) -> Option<glm::DVec2> {
        //TODO: Check for endpoints. Will have to run mtv around those as well
        //Current implementation assumes infinite line
        let len = fixed.1 - fixed.0;
        let normal = glm::normalize(len);
        let normal = glm::dvec2(-normal.y, normal.x);
        let distance = (len.y * self.center.x - len.x * self.center.y + fixed.1.x * fixed.0.y -
                        fixed.1.y * fixed.0.x) /
                       glm::length(len);
        if distance.abs() < self.radius {
            let normal = if distance < 0. { normal * -1. } else { normal };
            Some(glm::ext::normalize_to(normal, distance.abs() - self.radius))
        } else {
            None
        }
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
        assert_eq!(circle_a.mtv(&circle_b), None);
        assert!(!circle_b.intersects(&circle_a));
        assert_eq!(circle_b.mtv(&circle_a), None);
    }

    #[test]
    fn circle_circle_intersect() {
        let circle_a = Circle {
            radius: 3_f64,
            center: glm::dvec2(1_f64, 3_f64),
        };

        let circle_b = Circle {
            radius: 1_f64,
            center: glm::dvec2(1_f64, 5_f64),
        };

        assert!(circle_a.intersects(&circle_b));
        assert_eq!(circle_a.mtv(&circle_b), Some(glm::dvec2(0., -2.)));
        assert!(circle_b.intersects(&circle_a));
        assert_eq!(circle_b.mtv(&circle_a), Some(glm::dvec2(0., 2.)));
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

        assert!(!circle.contains(&glm::dvec2(3.5, 1.0)));
        assert!(!circle.intersects(&rectangle));
        assert_eq!(circle.mtv(&rectangle), None);
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

        assert!(circle.intersects(&rectangle));
        assert_eq!(circle.mtv(&rectangle), Some(glm::dvec2(0., -4.)));
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

        assert!(circle.intersects(&rectangle));
        assert_eq!(circle.mtv(&rectangle), Some(glm::dvec2(2.5, 0.)));
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

        assert!(circle.intersects(&rectangle));
        let mtv = circle.mtv(&rectangle);
        assert!(mtv.is_some());
        let mtv = mtv.unwrap();
        assert!(glm::length(mtv - glm::dvec2(-0.1, 0.)) < 0.00001);
    }

    #[test]
    fn rectangle_circle_touch_no_intersect() {
        let rectangle = Rectangle {
            dims: glm::dvec2(2., 2.),
            top_left: glm::dvec2(4., 2.),
        };

        let circle = Circle {
            radius: 2.,
            center: glm::dvec2(2., 3.),
        };

        assert!(!circle.intersects(&rectangle));
        assert_eq!(circle.mtv(&rectangle), None);
    }

    #[test]
    fn circle_intersects_line() {
        let line = (glm::dvec2(2., 2.), glm::dvec2(5., 2.));
        let circle = Circle {
            radius: 3.,
            center: glm::dvec2(3., 1.),
        };
        assert!(circle.intersects(&line));
        assert_eq!(circle.mtv(&line), Some(glm::dvec2(0., -2.)));
        let line = (glm::dvec2(2., -1.), glm::dvec2(5., -1.));
        assert!(circle.intersects(&line));
        assert_eq!(circle.mtv(&line), Some(glm::dvec2(0., 1.)));
    }
}
