use core::panic;
use std::fmt::Display;
use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait Vec3Properties {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn len(self) -> f64;
    fn len_squared(self) -> f64;
}

impl Vec3Properties for Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }

    fn len(self) -> f64 {
        f64::sqrt(self.len_squared())
    }

    fn len_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<u8> for Vec3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(
                "Can't access value with index {} from {}. Allowed indices are 0, 1 and 2.",
                index, self
            ),
        }
    }
}

impl IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(
                "Can't access value with index {} from {}. Allowed indices are 0, 1 and 2.",
                index, self
            ),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

#[test]
fn test_instantiation() {
    // When I instantiate a Vec3
    let vec3 = Vec3 {
        x: 24f64,
        y: 35f64,
        z: 46f64,
    };

    // Then I can read the values I instantiated it with
    assert_eq!(vec3.x, 24f64);
    assert_eq!(vec3.y, 35f64);
    assert_eq!(vec3.z, 46f64);
}

#[test]
fn test_display() {
    // Given a Vec3
    let vec3 = Vec3 {
        x: 24f64,
        y: 35f64,
        z: 46f64,
    };

    // When I format it as a string
    let formatted = format!("{}", vec3);

    // Then it displays the values
    assert_eq!("Vec3 { x: 24.0, y: 35.0, z: 46.0 }", formatted);
}

#[test]
fn test_negation() {
    // Given a Vec3
    let vec3 = Vec3 {
        x: 24f64,
        y: 35f64,
        z: 46f64,
    };

    // When I negate it
    let negated = -vec3;

    // Then its values are negated
    assert_eq!(negated.x, -24f64);
    assert_eq!(negated.y, -35f64);
    assert_eq!(negated.z, -46f64);
}

#[test]
fn test_indexing() {
    // Given a Vec3
    let vec3 = Vec3 {
        x: 24f64,
        y: 35f64,
        z: 46f64,
    };

    // When I access its values by indices
    // Then I can read its values
    assert_eq!(vec3[0], 24f64);
    assert_eq!(vec3[1], 35f64);
    assert_eq!(vec3[2], 46f64);
}

#[test]
fn test_mutable_indexing() {
    // Given a Vec3
    let mut vec3 = Vec3 {
        x: 24f64,
        y: 35f64,
        z: 46f64,
    };

    // When I write to its values by indices
    vec3[0] = 0f64;
    vec3[1] = 1f64;
    vec3[2] = 2f64;

    // Then I can read the values I wrote
    assert_eq!(vec3.x, 0f64);
    assert_eq!(vec3.y, 1f64);
    assert_eq!(vec3.z, 2f64);
}

#[test]
fn test_add_assign() {
    // Given two Vec3s
    let mut vec3_first = Vec3 {
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    let vec3_second = Vec3 {
        x: 3f64,
        y: 4f64,
        z: 5f64,
    };

    // When I add assign the second to the first
    vec3_first += vec3_second;

    // Then the first contains the sum of the addition
    assert_eq!(vec3_first.x, 3f64);
    assert_eq!(vec3_first.y, 5f64);
    assert_eq!(vec3_first.z, 7f64);
}

#[test]
fn test_mul_assign() {
    // Given two Vec3s
    let mut vec3_first = Vec3 {
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    let vec3_second = Vec3 {
        x: 3f64,
        y: 4f64,
        z: 5f64,
    };

    // When I multiply assign the first by the second
    vec3_first *= vec3_second;

    // Then the first contains the product of the multiplication
    assert_eq!(vec3_first.x, 0f64);
    assert_eq!(vec3_first.y, 4f64);
    assert_eq!(vec3_first.z, 10f64);
}

#[test]
fn test_div_assign() {
    // Given two Vec3s
    let mut vec3_first = Vec3 {
        x: 0f64,
        y: 1f64,
        z: 2f64,
    };
    let vec3_second = Vec3 {
        x: 3f64,
        y: 4f64,
        z: 5f64,
    };

    // When I divide assign the first by the second
    vec3_first /= vec3_second;

    // Then the first contains the result of the division
    assert_eq!(vec3_first.x, 0f64);
    assert_eq!(vec3_first.y, 0.25f64);
    assert_eq!(vec3_first.z, 0.4f64);
}

#[test]
fn test_len() {
    // Given a Vec3
    let vec3 = Vec3 {
        x: 24.0,
        y: 35.0,
        z: 46.0,
    };

    // When I get the length
    let len = vec3.len();

    // Then it is equal to the distance from { 0, 0, 0 } to its values
    assert_eq!(62.5859409132754, len);
}

#[test]
fn test_len_squared() {
    // Given a Vec3
    let vec3 = Vec3 {
        x: 24.0,
        y: 35.0,
        z: 46.0,
    };

    // When I get the length squared
    let len_squared = vec3.len_squared();

    // Then it is equal to the sum of all of its values sqaured
    assert_eq!(3917.0, len_squared);
}
