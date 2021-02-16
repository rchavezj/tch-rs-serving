use tch::Tensor;
extern crate tch;

fn main() {
    let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
    t.print();
    let t = t * 2;
    t.print();
}