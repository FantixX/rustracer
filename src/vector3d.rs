use core::fmt;
use std::ops;
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
}
impl ops::Add<&Vec3> for Vec3 {
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::from(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    type Output = Vec3;
}

impl ops::Sub<&Vec3> for Vec3 {
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3::from(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    type Output = Vec3;
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Mul<&Vec3> for Vec3 {
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::from(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    type Output = Vec3;
}

impl ops::Div<&Vec3> for Vec3 {
    fn div(self, other: &Vec3) -> Vec3 {
        Vec3::from(self.x / other.x, self.y / other.y, self.z / other.z)
    }

    type Output = Vec3;
}

impl ops::Mul<&f64> for Vec3 {
    fn mul(self, other: &f64) -> Self::Output {
        Vec3::from(self.x * other, self.y * other, self.z * other)
    }

    type Output = Vec3;
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x: {x}, y: {y}, z: {z}",
            x = self.x,
            y = self.y,
            z = self.z
        )
    }
}

pub fn scalar_product(vec1: &Vec3, vec2: &Vec3) -> f64 {
    vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
}

pub fn cross_product(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
    return Vec3::from(
        vec1.y * vec2.z - vec1.z * vec2.y,
        vec1.z * vec2.x - vec1.x * vec2.z,
        vec1.x * vec2.y - vec1.y * vec2.x,
    );
}

pub type Color = Vec3;
pub type Point3 = Vec3;
