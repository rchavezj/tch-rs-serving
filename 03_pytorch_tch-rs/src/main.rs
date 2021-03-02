extern crate tch;
use anyhow::Result;

mod mnist_conv;
mod mnist_linear;
mod mnist_nn;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let model = if args.len() < 2 {
        None
    } else {
        Some(args[1].as_str())
    };
    match model {
        None => mnist_nn::run(),
        Some("linear") => mnist_linear::run(),
        Some("conv") => mnist_conv::run(),
        Some(_) => mnist_nn::run(),
    }
}