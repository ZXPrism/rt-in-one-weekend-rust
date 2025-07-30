use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Sub};

pub mod alias;
pub use alias::*;

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

#[derive(Debug, Copy, Clone)]
pub struct Vector<Ty, const N: usize> {
    data: [Ty; N],
}

impl<Ty, const N: usize> Vector<Ty, N>
where
    Ty: Copy,
{
    pub fn new(init_values: [Ty; N]) -> Self {
        Vector { data: init_values }
    }
}

impl<Ty, const N: usize> Vector<Ty, N>
where
    Ty: Default + Copy,
{
    pub fn zeros() -> Self {
        Vector {
            data: [Ty::default(); N],
        }
    }
}

impl<Ty, const N: usize> Vector<Ty, N>
where
    Ty: Default + Copy + Mul<Output = Ty> + AddAssign,
{
    pub fn dot_product(lhs: Vector<Ty, N>, rhs: Vector<Ty, N>) -> Ty {
        let mut res = Ty::default();

        for i in 0..N {
            res += lhs.data[i] * rhs.data[i];
        }

        res
    }
}

impl<Ty> Vector<Ty, 3>
where
    Ty: Copy + Mul<Output = Ty> + Sub<Output = Ty>,
{
    pub fn cross_product(lhs: Vector<Ty, 3>, rhs: Vector<Ty, 3>) -> Vector<Ty, 3> {
        Vector::new([
            lhs.data[1] * rhs.data[2] - lhs.data[2] * rhs.data[1],
            lhs.data[2] * rhs.data[0] - lhs.data[0] * rhs.data[2],
            lhs.data[0] * rhs.data[1] - lhs.data[1] * rhs.data[0],
        ])
    }
}

impl<Ty, const N: usize> Vector<Ty, N>
where
    Ty: Default + Copy + Mul<Output = Ty> + AddAssign + Sqrt,
{
    pub fn length(self) -> Ty {
        let mut res = Ty::default();

        for i in 0..N {
            res += self.data[i] * self.data[i];
        }

        res.sqrt()
    }
}

impl<Ty, const N: usize> Vector<Ty, N>
where
    Ty: Default + Copy + Mul<Output = Ty> + Div<Output = Ty> + AddAssign + Sqrt,
{
    pub fn unit_vec(self) -> Self {
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

impl<Ty, const N: usize> Add<Self> for Vector<Ty, N>
where
    Ty: Add<Output = Ty> + Default + Copy,
{
    type Output = Vector<Ty, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Vector::zeros();

        for i in 0..N {
            res.data[i] = self.data[i] + rhs.data[i];
        }

        res
    }
}

impl<Ty, const N: usize> AddAssign<Self> for Vector<Ty, N>
where
    Ty: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<Ty, const N: usize> Sub<Self> for Vector<Ty, N>
where
    Ty: Sub<Output = Ty> + Default + Copy,
{
    type Output = Vector<Ty, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Vector::zeros();

        for i in 0..N {
            res.data[i] = self.data[i] - rhs.data[i];
        }

        res
    }
}

impl<Ty, const N: usize> Mul<Ty> for Vector<Ty, N>
where
    Ty: Mul<Output = Ty> + Default + Copy,
{
    type Output = Vector<Ty, N>;

    fn mul(self, rhs: Ty) -> Self::Output {
        let mut res = Vector::zeros();

        for i in 0..N {
            res.data[i] = self.data[i] * rhs;
        }

        res
    }
}

impl<Ty, const N: usize> MulAssign<Ty> for Vector<Ty, N>
where
    Ty: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Ty) {
        for i in 0..N {
            self.data[i] *= rhs;
        }
    }
}

impl<Ty, const N: usize> Div<Ty> for Vector<Ty, N>
where
    Ty: Div<Output = Ty> + Default + Copy,
{
    type Output = Vector<Ty, N>;

    fn div(self, rhs: Ty) -> Self::Output {
        let mut res = Vector::zeros();

        for i in 0..N {
            res.data[i] = self.data[i] / rhs;
        }

        res
    }
}
