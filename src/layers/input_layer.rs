use crate::layer::{Evolve, Layer};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct InputLayer<L> {
    input_type: PhantomData<L>,
}

impl<L> InputLayer<L> {
    pub fn new() -> Self {
        Self {
            input_type: PhantomData,
        }
    }
}

impl<L: std::fmt::Debug> Layer for InputLayer<L>
where
    L: Clone,
{
    type Input = L;
    type Output = L;

    fn activate(&self, input: &Self::Input) -> Self::Output {
        input.clone()
    }
}

impl<L> Evolve for InputLayer<L> {
    fn evolve(self, _other: Self) -> Self {
        self
    }
}

impl<L> std::fmt::Display for InputLayer<L> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(formatter, "Input Layer")?;
        writeln!(formatter)?;

        Ok(())
    }
}
