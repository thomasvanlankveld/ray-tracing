use crate::point3::Point3;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    #[allow(dead_code)]
    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[test]
fn test_point_at_ray() {
    // Given a ray
    let origin = Point3::new(1., 2., 3.);
    let direction = Vec3::new(-1., -1., -1.);
    let ray = Ray::new(origin, direction);

    // When I ask for a point along that ray
    let point = ray.at(3.);

    // Then I get the correct point
    assert_eq!(point, Point3::new(-2., -1., 0.));
}
