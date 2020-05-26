use crate::layer::Layer;
use random::Source;
use rayon::prelude::*;
use std::collections::HashMap;

type SynapseID = u32;

#[derive(Debug, Clone)]
struct Axon {
    synapse: SynapseID,
    weight: f32,
}

#[derive(Debug, Clone)]
enum Synapse {
    Synapse { axons: Vec<Axon> },
    Input(usize),
}

impl Synapse {
    pub fn activate(&self, neat_layer: &NeatLayer, input: &Vec<f32>) -> f32 {
        match self {
            Self::Synapse { axons } => axons
                .par_iter()
                .map(|axon| {
                    neat_layer
                        .synapses
                        .get(&axon.synapse)
                        .expect("invalid synapse id")
                        .activate(neat_layer, input) * axon.weight
                })
                .sum(),

            Self::Input(index) => input[*index],
        }
    }
}

#[derive(Debug)]
pub struct NeatLayer {
    synapses: HashMap<SynapseID, Synapse>,
    output_synapses: Vec<SynapseID>,
    layers: Vec<Vec<SynapseID>>,
}

impl NeatLayer {
    pub fn random<R>(input_len: usize, output_len: usize, random: &mut R) -> Self
    where
        R: Source,
    {
        let mut synapses = HashMap::new();
        let mut next_id = 0;
        let mut input_synapses = Vec::with_capacity(input_len);

        for i in 0..input_len {
            synapses.insert(next_id, Synapse::Input(i));
            input_synapses.push(next_id);
            next_id += 1;
        }

        let mut output_synapses = Vec::with_capacity(output_len);

        for _ in 0..output_len {
            synapses.insert(
                next_id,
                Synapse::Synapse {
                    axons: input_synapses
                        .iter()
                        .map(|id| Axon {
                            synapse: *id,
                            weight: random.read::<f32>() * 2.0 - 1.0,
                        })
                        .collect(),
                },
            );

            output_synapses.push(next_id);

            next_id += 1;
        }

        Self {
            synapses,
            output_synapses,
            layers: Vec::new(),
        }
    }
}

impl Layer for NeatLayer {
    type Input = Vec<f32>;
    type Output = Vec<f32>;

    fn activate(&self, input: &Self::Input) -> Self::Output {
        self.output_synapses
            .par_iter()
            .map(|id| {
                let synapse = self.synapses.get(id).expect("invalid synapse id");

                synapse.activate(self, input)
            })
            .collect()
    }
}
