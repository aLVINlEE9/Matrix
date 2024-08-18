use matrix::types::{matrix::Matrix, vector::Vector};

fn main() {
    // Vector operations
    let mut u = Vector::from(vec![2.0, 3.0]);
    let v = Vector::from(vec![5.0, 7.0]);
    u.add(&v);
    println!("{}", u);

    let mut u = Vector::from(vec![2.0, 3.0]);
    let v = Vector::from(vec![5.0, 7.0]);
    u.sub(&v);
    println!("{}", u);

    let mut u = Vector::from(vec![2.0, 3.0]);
    u.scl(2.0);
    println!("{}", u);

    // Matrix operations
    let mut u = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let v = Matrix::from(vec![vec![7.0, 4.0], vec![-2.0, 2.0]]);
    u.add(&v);
    println!("{}", u);

    let mut u = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let v = Matrix::from(vec![vec![7.0, 4.0], vec![-2.0, 2.0]]);
    u.sub(&v);
    println!("{}", u);

    let mut u = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    u.scl(2.0);
    println!("{}", u);
}
