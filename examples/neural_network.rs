extern crate gulper;

use gulper::layer::Layer;
use gulper::layers::*;
use gulper::random;

fn main() {
    let mut random = random::Default::new();

    let network = InputLayer::new().layer(NeatLayer::random(2, 3, &mut random));

    println!("{:?}", network);
    println!("{:?}", network.activate(&vec![0.5, -1.0]));
}
