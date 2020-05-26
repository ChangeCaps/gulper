pub trait Layer: Sized + std::fmt::Debug {
    type Input;
    type Output;

    fn activate(&self, input: &Self::Input) -> Self::Output;

    fn layer<L0>(self, layer: L0) -> LayerContainer<Self, L0>
    where
        L0: Layer<Input = Self::Output>,
    {
        LayerContainer {
            layer: self,
            next: layer,
        }
    }
}

pub trait Evolve {
    fn evolve(self, other: Self) -> Self;
}

#[derive(Debug)]
pub struct LayerContainer<L, N> {
    layer: L,
    next: N,
}

impl<L, N> Layer for LayerContainer<L, N>
where
    L: Layer,
    N: Layer<Input = L::Output>,
{
    type Input = L::Input;
    type Output = N::Output;

    fn activate(&self, input: &Self::Input) -> Self::Output {
        let input = self.layer.activate(input);

        self.next.activate(&input)
    }
}

impl<L, N> Evolve for LayerContainer<L, N>
where
    L: Evolve,
    N: Evolve,
{
    fn evolve(self, other: Self) -> Self {
        Self {
            layer: self.layer.evolve(other.layer),
            next: self.next.evolve(other.next),
        }
    }
}

impl<L, N> std::fmt::Display for LayerContainer<L, N>
where
    L: std::fmt::Display,
    N: std::fmt::Display,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(formatter, "{}", self.layer)?;
        writeln!(formatter, "{}", self.next)?;

        Ok(())
    }
}
