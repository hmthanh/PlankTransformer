//! Tensor operations

use bytemuck::{Pod, Zeroable};

/// Simple tensor structure for neural network operations
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TensorDescriptor {
    pub shape: [u32; 4],
    pub size: u32,
}

/// Tensor data holder
#[derive(Debug, Clone)]
pub struct Tensor {
    pub data: Vec<f32>,
    pub shape: Vec<usize>,
}

impl Tensor {
    /// Create a new tensor with given shape
    pub fn new(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        Self {
            data: vec![0.0; size],
            shape,
        }
    }

    /// Create tensor from data
    pub fn from_data(data: Vec<f32>, shape: Vec<usize>) -> Self {
        assert_eq!(data.len(), shape.iter().product::<usize>());
        Self { data, shape }
    }

    /// Get total number of elements
    pub fn size(&self) -> usize {
        self.shape.iter().product::<usize>()
    }

    /// Get descriptor for GPU operations
    pub fn descriptor(&self) -> TensorDescriptor {
        let mut shape_arr = [1u32; 4];
        for (i, &s) in self.shape.iter().take(4).enumerate() {
            shape_arr[i] = s as u32;
        }
        TensorDescriptor {
            shape: shape_arr,
            size: self.size() as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let tensor = Tensor::new(vec![2, 3, 4]);
        assert_eq!(tensor.size(), 24);
    }

    #[test]
    fn test_tensor_from_data() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let tensor = Tensor::from_data(data, vec![2, 2]);
        assert_eq!(tensor.size(), 4);
    }
}
