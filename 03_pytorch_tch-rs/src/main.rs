extern crate tch;
use tch::Tensor;

fn main() {
    let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
    // let _t = Tensor;
    // let t = t * 2;
    t.print();
}