#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, magnitude: f64) -> Self {
        Self::new(self.x * magnitude, self.y * magnitude, self.z * magnitude)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vector: Vec3) -> Vec3 {
        vector * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(0.1, 0.2, 0.3);
        let c = Vec3::new(1.1, 2.2, 3.3);
        assert_eq!(a + b, c);
    }

    #[test]
    fn subtraction() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(0.1, 0.2, 0.3);
        let c = Vec3::new(0.9, 1.8, 2.7);
        assert_eq!(a - b, c);
    }

    #[test]
    fn scalar_multiplication() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(a * 3.0, b);
        assert_eq!(3.0 * a, b);
    }
}
