use std::ops::Index;

pub struct Matrix {
    size: usize,
    data: Vec<u32>,
}

impl Index<(usize, usize)> for Matrix {
    type Output = u32;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.size * index.0 + index.1]
    }
}

impl Matrix {
    pub fn from_slice(a: &[u32], size: usize) -> Self {
        let data = a[0..(size * size)].to_vec();
        Matrix { size, data }
    }
}
