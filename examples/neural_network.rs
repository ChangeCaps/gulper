extern crate gulper;

use gulper::layer::Layer;
use gulper::layers::*;

fn main() {
    let network = InputLayer::new()
        .layer(DenseLayer::new(2, 2))
        .layer(DenseLayer::new(2, 3));

    println!("{:?}", network.activate(&vec![0.3, 0.6]));
}
