use crate::ArrayError;

/// A multidimensional array.
#[derive(Debug, Clone, PartialEq)]
pub struct Array<T> {
    data: Vec<T>,
    shape: Vec<usize>,
}

impl<T> Array<T> {
    /// Creates an array with the given shape.
    pub fn new(data: Vec<T>, shape: Vec<usize>) -> Result<Self, ArrayError> {
        let expected_length: usize = shape.iter().product();

        if data.len() != expected_length {
            return Err(ArrayError::ShapeMismatch);
        }

        Ok(Self { data, shape })
    }

    /// Creates a one-dimensional array.
    pub fn vector(data: Vec<T>) -> Self {
        let length = data.len();

        Self::new(data, vec![length]).expect("vector shape is calculated from its data")
    }

    /// Creates a two-dimensional array.
    pub fn matrix(data: Vec<T>, rows: usize, columns: usize) -> Result<Self, ArrayError> {
        Self::new(data, vec![rows, columns])
    }

    /// Returns the array data.
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Returns the array shape.
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    /// Returns the number of dimensions.
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the array contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
