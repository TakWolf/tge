use super::Number;
use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector<N: Number = f32> {
    pub x: N,
    pub y: N,
}

pub type Position<N = f32> = Vector<N>;

impl<N: Number> Vector<N> {
    pub fn new(x: N, y: N) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(N::zero(), N::zero())
    }

    pub fn none() -> Option<Self> {
        None
    }
}

impl<N: Number> Add for Vector<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<N: Number> Sub for Vector<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<N: Number> AddAssign for Vector<N> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<N: Number> SubAssign for Vector<N> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<N: Number> From<(N, N)> for Vector<N> {
    fn from((x, y): (N, N)) -> Self {
        Self::new(x, y)
    }
}

impl<N: Number> Into<(N, N)> for Vector<N> {
    fn into(self) -> (N, N) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn test_operator() {
        let mut vec = Vector::<f32>::new(100.0, 50.0);
        vec = vec + Vector::<f32>::new(40.0, 20.0);
        assert_eq!(vec, Vector::<f32>::new(140.0, 70.0));
        vec = vec - Vector::<f32>::new(40.0, 70.0);
        assert_eq!(vec, Vector::<f32>::new(100.0, 0.0));
        vec += Vector::<f32>::new(20.0, 50.0);
        assert_eq!(vec, Vector::new(120.0, 50.0));
        vec -= Vector::<f32>::new(80.0, 10.0);
        assert_eq!(vec, Vector::new(40.0, 40.0));
    }
}
