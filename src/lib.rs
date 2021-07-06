use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl ops::Add for Vector {
    type Output = Self;
    fn add(self, v: Self) -> Self {
        Self::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl ops::Sub for Vector {
    type Output = Self;
    fn sub(self, v: Self) -> Self {
        Self::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, x: f64) -> Self {
        Self::new(x * self.x, x * self.y, x * self.z)
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, v: Vector) -> Vector {
        v * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(0.1, 0.2, 0.3);
        let c = Vector::new(1.1, 2.2, 3.3);
        assert_eq!(a + b, c);
    }

    #[test]
    fn subtraction() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(0.1, 0.2, 0.3);
        let c = Vector::new(0.9, 1.8, 2.7);
        assert_eq!(a - b, c);
    }

    #[test]
    fn scalar_multiplication() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(3.0, 6.0, 9.0);
        assert_eq!(a * 3.0, b);
        assert_eq!(3.0 * a, b);
    }
}
