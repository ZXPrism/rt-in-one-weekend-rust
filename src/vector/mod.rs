use std::ops::{Add, Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct Vector<Ty, const N: usize> {
    data: [Ty; N],
}

impl<Ty, const N: usize> Index<usize> for Vector<Ty, N> {
    type Output = Ty;

    fn index(&self, index: usize) -> &Ty {
        return &self.data[index];
    }
}

impl<Ty, const N: usize> IndexMut<usize> for Vector<Ty, N> {
    fn index_mut(&mut self, index: usize) -> &mut Ty {
        return &mut self.data[index];
    }
}

// TODO:
// ??? 为什么 impl for 的对象也可以加引用？？
// add rhs 为什么默认不带引用？？
impl<Ty, const N: usize> Add<&Self> for &Vector<Ty, N> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {}
}

#[allow(dead_code)]
pub type Vector2f = Vector<f32, 2>;

#[allow(dead_code)]
pub type Vector3f = Vector<f32, 3>;

#[allow(dead_code)]
pub type Vector2d = Vector<f64, 2>;

#[allow(dead_code)]
pub type Vector3d = Vector<f64, 3>;
