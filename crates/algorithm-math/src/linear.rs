#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, String> {
        if data.len() != rows * cols { return Err("matrix dimensions do not match data length".into()); }
        Ok(Self { rows, cols, data })
    }

    pub fn zeros(rows: usize, cols: usize) -> Self { Self { rows, cols, data: vec![0.0; rows * cols] } }

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        (row < self.rows && col < self.cols).then(|| self.data[row * self.cols + col])
    }

    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for r in 0..self.rows { for c in 0..self.cols { out.data[c * out.cols + r] = self.data[r * self.cols + c]; } }
        out
    }

    pub fn mul(&self, rhs: &Self) -> Result<Self, String> {
        if self.cols != rhs.rows { return Err("matrix dimensions are incompatible".into()); }
        let mut out = Self::zeros(self.rows, rhs.cols);
        for i in 0..self.rows { for k in 0..self.cols { for j in 0..rhs.cols { out.data[i * rhs.cols + j] += self.data[i * self.cols + k] * rhs.data[k * rhs.cols + j]; } } }
        Ok(out)
    }
}
