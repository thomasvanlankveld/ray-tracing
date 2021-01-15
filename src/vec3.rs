use core::panic;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg};

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

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

#[test]
fn test_instantiation() {
    // When I instantiate a Vec3
    let vec3 = Vec3::new(24., 35., 46.);

    // Then I can read the values I instantiated it with
    assert_eq!(vec3.x, 24.);
    assert_eq!(vec3.y, 35.);
    assert_eq!(vec3.z, 46.);
}

#[test]
fn test_display() {
    // Given a Vec3
    let vec3 = Vec3::new(24., 35., 46.);

    // When I format it as a string
    let formatted = format!("{}", vec3);

    // Then it displays the values
    assert_eq!("Vec3 { x: 24.0, y: 35.0, z: 46.0 }", formatted);
}

#[test]
fn test_negation() {
    // Given a Vec3
    let vec3 = Vec3::new(24., 35., 46.);

    // When I negate it
    let negated = -vec3;

    // Then its values are negated
    assert_eq!(negated.x, -24.);
    assert_eq!(negated.y, -35.);
    assert_eq!(negated.z, -46.);
}

#[test]
fn test_indexing() {
    // Given a Vec3
    let vec3 = Vec3::new(24., 35., 46.);

    // When I access its values by indices
    // Then I can read its values
    assert_eq!(vec3[0], 24.);
    assert_eq!(vec3[1], 35.);
    assert_eq!(vec3[2], 46.);
}

#[test]
fn test_mutable_indexing() {
    // Given a Vec3
    let mut vec3 = Vec3::new(24., 35., 46.);

    // When I write to its values by indices
    vec3[0] = 0.;
    vec3[1] = 1.;
    vec3[2] = 2.;

    // Then I can read the values I wrote
    assert_eq!(vec3.x, 0.);
    assert_eq!(vec3.y, 1.);
    assert_eq!(vec3.z, 2.);
}

#[test]
fn test_add() {
    // Given two Vec3s
    let vec3_first = Vec3::new(0., 1., 2.);
    let vec3_second = Vec3::new(3., 4., 5.);

    // When I add them together
    let sum = vec3_first + vec3_second;

    // Then the first contains the sum of the addition
    assert_eq!(sum.x, 3.);
    assert_eq!(sum.y, 5.);
    assert_eq!(sum.z, 7.);
}

#[test]
fn test_add_assign() {
    // Given two Vec3s
    let mut vec3_first = Vec3::new(0., 1., 2.);
    let vec3_second = Vec3::new(3., 4., 5.);

    // When I add assign the second to the first
    vec3_first += vec3_second;

    // Then the first contains the sum of the addition
    assert_eq!(vec3_first.x, 3.);
    assert_eq!(vec3_first.y, 5.);
    assert_eq!(vec3_first.z, 7.);
}

#[test]
fn test_mul() {
    // Given two Vec3s
    let vec3_first = Vec3::new(0., 1., 2.);
    let vec3_second = Vec3::new(3., 4., 5.);

    // When I multiply them
    let product = vec3_first * vec3_second;

    // Then the first contains the product of the multiplication
    assert_eq!(product.x, 0.);
    assert_eq!(product.y, 4.);
    assert_eq!(product.z, 10.);
}

#[test]
fn test_mul_float() {
    // Given a Vec3
    let vec3 = Vec3::new(24., 35., 46.);

    // When I multiply it by 0.75
    let product = vec3 * 0.75;

    // Then the first contains the product of the multiplication
    assert_eq!(product.x, 18.);
    assert_eq!(product.y, 26.25);
    assert_eq!(product.z, 34.5);
}

#[test]
fn test_mul_assign() {
    // Given two Vec3s
    let mut vec3_first = Vec3::new(0., 1., 2.);
    let vec3_second = Vec3::new(3., 4., 5.);

    // When I multiply assign the first by the second
    vec3_first *= vec3_second;

    // Then the first contains the product of the multiplication
    assert_eq!(vec3_first.x, 0.);
    assert_eq!(vec3_first.y, 4.);
    assert_eq!(vec3_first.z, 10.);
}

#[test]
fn test_div() {
    // Given two Vec3s
    let vec3_first = Vec3::new(0., 1., 2.);
    let vec3_second = Vec3::new(3., 4., 5.);

    // When I divide the first by the second
    let result = vec3_first / vec3_second;

    // Then the first contains the result of the division
    assert_eq!(result.x, 0.);
    assert_eq!(result.y, 0.25);
    assert_eq!(result.z, 0.4);
}

#[test]
fn test_div_assign() {
    // Given two Vec3s
    let mut vec3_first = Vec3::new(0., 1., 2.);
    let vec3_second = Vec3::new(3., 4., 5.);

    // When I divide assign the first by the second
    vec3_first /= vec3_second;

    // Then the first contains the result of the division
    assert_eq!(vec3_first.x, 0.);
    assert_eq!(vec3_first.y, 0.25);
    assert_eq!(vec3_first.z, 0.4);
}

#[test]
fn test_len() {
    // Given a Vec3
    let vec3 = Vec3::new(24., 35., 46.);

    // When I get the length
    let len = vec3.len();

    // Then it is equal to the distance from { 0, 0, 0 } to its values
    assert_eq!(62.5859409132754, len);
}

#[test]
fn test_len_squared() {
    // Given a Vec3
    let vec3 = Vec3::new(24., 35., 46.);

    // When I get the length squared
    let len_squared = vec3.len_squared();

    // Then it is equal to the sum of all of its values sqaured
    assert_eq!(3917., len_squared);
}
