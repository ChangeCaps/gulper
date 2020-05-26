use crate::layer::Layer;
use random::Source;
use rayon::prelude::*;

#[derive(Debug)]
pub struct DenseLayer {
    pub input_len: usize,
    pub output_len: usize,
    pub weights: Vec<Vec<f32>>,
}

impl DenseLayer {
    pub fn new(input_len: usize, output_len: usize) -> Self {
        Self {
            input_len,
            output_len,
            weights: vec![vec![0.5; input_len]; output_len],
        }
    }

    pub fn random<R>(input_len: usize, output_len: usize, random: &mut R) -> Self
    where
        R: Source,
    {
        let mut weights = Vec::new();

        for _ in 0..output_len {
            let mut _weights = Vec::new();

            for _ in 0..input_len {
                _weights.push(random.read::<f32>() * 2.0 - 1.0);
            }

            weights.push(_weights);
        }

        Self {
            input_len,
            output_len,
            weights,
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
                    .sum::<f32>()
                    .max(0.0)
            })
            .collect()
    }
}

impl std::fmt::Display for DenseLayer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let max_len = self.input_len.max(self.output_len);

        writeln!(formatter, "Dense Layer:")?;

        if max_len <= 10 {
            write!(formatter, "[")?;

            let input_lines = max_len - self.input_len;

            for _ in 0..input_lines {
                write!(formatter, "-")?;
            }

            for _ in 0..self.input_len - 1 {
                write!(formatter, "O-")?;
            }

            write!(formatter, "O")?;

            for _ in 0..input_lines {
                write!(formatter, "-")?;
            }

            writeln!(formatter, "]")?;
            writeln!(formatter)?;

            write!(formatter, "[")?;

            let output_lines = max_len - self.output_len;

            for _ in 0..output_lines {
                write!(formatter, "-")?;
            }

            for _ in 0..self.output_len - 1 {
                write!(formatter, "O-")?;
            }

            write!(formatter, "O")?;

            for _ in 0..output_lines {
                write!(formatter, "-")?;
            }

            writeln!(formatter, "]")?;
        } else {
            writeln!(formatter, "Input Length: {}", self.input_len)?;
            writeln!(formatter, "Output Length: {}", self.output_len)?;
        }

        writeln!(formatter)?;

        Ok(())
    }
}
