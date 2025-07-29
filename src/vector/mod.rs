use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Sub};

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct Vector<Ty, const N: usize> {
    data: [Ty; N],
}

impl<Ty: Copy, const N: usize> Vector<Ty, N> {
    pub fn new(init_values: [Ty; N]) -> Self {
        Vector { data: init_values }
    }
}

impl<Ty: Default + Copy, const N: usize> Vector<Ty, N> {
    pub fn zeros() -> Self {
        Vector {
            data: [Ty::default(); N],
        }
    }
}

impl<Ty: Default + Copy + Mul<Output = Ty> + AddAssign, const N: usize> Vector<Ty, N> {
    pub fn dot_product(lhs: &Vector<Ty, N>, rhs: &Vector<Ty, N>) -> Ty {
        let mut res = Ty::default();

        for i in 0..N {
            res += lhs.data[i] * rhs.data[i];
        }

        res
    }
}

impl<Ty: Copy + Mul<Output = Ty> + Sub<Output = Ty>> Vector<Ty, 3> {
    pub fn cross_product(lhs: &Vector<Ty, 3>, rhs: &Vector<Ty, 3>) -> Vector<Ty, 3> {
        Vector::new([
            lhs.data[1] * rhs.data[2] - lhs.data[2] * rhs.data[1],
            lhs.data[2] * rhs.data[0] - lhs.data[0] * rhs.data[2],
            lhs.data[0] * rhs.data[1] - lhs.data[1] * rhs.data[0],
        ])
    }
}

impl<Ty: Default + Copy + Mul<Output = Ty> + AddAssign + Sqrt, const N: usize> Vector<Ty, N> {
    pub fn length(&self) -> Ty {
        let mut res = Ty::default();

        for i in 0..N {
            res += self.data[i] * self.data[i];
        }

        res.sqrt()
    }
}

impl<Ty: Default + Copy + Mul<Output = Ty> + Div<Output = Ty> + AddAssign + Sqrt, const N: usize>
    Vector<Ty, N>
{
    pub fn unit_vec(&self) -> Self {
        self / Vector::length(self) // may divide by zero
    }
}

impl<Ty, const N: usize> Index<usize> for Vector<Ty, N> {
    type Output = Ty;

    fn index(&self, index: usize) -> &Ty {
        &self.data[index]
    }
}

impl<Ty, const N: usize> IndexMut<usize> for Vector<Ty, N> {
    fn index_mut(&mut self, index: usize) -> &mut Ty {
        &mut self.data[index]
    }
}

impl<Ty: Add<Output = Ty> + Default + Copy, const N: usize> Add<Self> for &Vector<Ty, N> {
    type Output = Vector<Ty, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Vector::zeros();

        for i in 0..N {
            res.data[i] = self.data[i] + rhs.data[i];
        }

        res
    }
}

impl<Ty: Sub<Output = Ty> + Default + Copy, const N: usize> Sub<Self> for &Vector<Ty, N> {
    type Output = Vector<Ty, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Vector::zeros();

        for i in 0..N {
            res.data[i] = self.data[i] - rhs.data[i];
        }

        res
    }
}

impl<Ty: Div<Output = Ty> + Default + Copy, const N: usize> Div<Ty> for &Vector<Ty, N> {
    type Output = Vector<Ty, N>;

    fn div(self, rhs: Ty) -> Self::Output {
        let mut res = Vector::zeros();

        for i in 0..N {
            res.data[i] = self.data[i] / rhs;
        }

        res
    }
}

#[allow(dead_code)]
pub type Vector2f = Vector<f32, 2>;

#[allow(dead_code)]
pub type Vector3f = Vector<f32, 3>;

#[allow(dead_code)]
pub type Vector2d = Vector<f64, 2>;

#[allow(dead_code)]
pub type Vector3d = Vector<f64, 3>;
