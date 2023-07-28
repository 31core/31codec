#[derive(Default, Debug, Clone)]
pub struct Matrix<E: Copy + Default> {
    elements: Vec<E>,
    size: usize,
}

impl<E: Copy + Default> Matrix<E> {
    pub fn new(size: usize) -> Self {
        let mut elements = Vec::new();
        for _ in 0..size.pow(2) {
            elements.push(E::default());
        }
        Self { elements, size }
    }
    pub fn from(elements: &[E], size: usize) -> Self {
        Self {
            elements: elements.to_vec(),
            size,
        }
    }
    pub fn get(&self, i: usize, j: usize) -> E {
        self.elements[j * self.size + i]
    }
    pub fn set(&mut self, i: usize, j: usize, value: E) {
        self.elements[j * self.size + i] = value;
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /** convert elements in matrix */
    pub fn convert<F, T>(&self, conv: F) -> Matrix<T>
    where
        F: Fn(E) -> T,
        T: Copy + Default,
    {
        let mut result: Matrix<T> = Matrix::new(self.size);
        for i in 0..self.elements.len() {
            result.elements[i] = conv(self.elements[i]);
        }
        result
    }
}
