use matrix::function::lerp::lerp;
use matrix::types::matrix::Matrix;

fn main() {
    println!("{:.1}", lerp(0.0, 1.0, 0.0));
    println!("{:.1}", lerp(0.0, 1.0, 1.0));
    println!("{:.1}", lerp(0.0, 1.0, 0.5));
    println!("{:.1}", lerp(21.0, 42.0, 0.3));

    let u = Matrix::from(vec![vec![2.0, 1.0]]);
    let v = Matrix::from(vec![vec![4.0, 2.0]]);
    println!("{}", lerp(u, v, 0.3));

    let a = Matrix::from(vec![vec![2.0, 1.0], vec![3.0, 4.0]]);
    let b = Matrix::from(vec![vec![20.0, 10.0], vec![30.0, 40.0]]);
    println!("{}", lerp(a, b, 0.5));
}
