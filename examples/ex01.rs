use matrix::function::linear_combination::linear_combination;
use matrix::types::vector::Vector;

fn main() {
    let e1 = Vector::from(vec![1.0, 0.0, 0.0]);
    let e2 = Vector::from(vec![0.0, 1.0, 0.0]);
    let e3 = Vector::from(vec![0.0, 0.0, 1.0]);

    let v1 = Vector::from(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::from(vec![0.0, 10.0, -100.0]);

    println!("{}", linear_combination(&[e1, e2, e3], &[10.0, -2.0, 0.5]));

    println!("{}", linear_combination(&[v1, v2], &[10.0, -2.0]));
}
