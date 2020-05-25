use crate::layer::Layer;
use rayon::prelude::*;

#[derive(Debug)]
pub struct DenseLayer {
    pub weights: Vec<Vec<f32>>,
}

impl DenseLayer {
    pub fn new(input_len: usize, output_len: usize) -> Self {
        Self {
            weights: vec![vec![0.5; input_len]; output_len],
        }
    }
}

impl Layer for DenseLayer {
    type Input = Vec<f32>;
    type Output = Vec<f32>;

    fn activate(&self, input: &Self::Input) -> Self::Output {
        self.weights
            .par_iter()
            .map(|weights| {
                weights
                    .par_iter()
                    .enumerate()
                    .map(|(index, weight)| *weight * input[index])
                    .sum()
            })
            .collect()
    }
}
