use core::panic;
use std::fmt::Display;
use std::ops::{Index, IndexMut, Neg};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait Vec3Properties {
    fn new(x: f64, y: f64, z: f64) -> Self;
}

impl Vec3Properties for Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
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
