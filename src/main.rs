use crate::matrix::Matrix2x2;

pub mod matrix;

fn main() {
    let mut m = Matrix2x2::new(
        [[90_000., 300.], [360_000., 600.]],
        [620., 0.],
    );
    println!("m = {:#}", m);
    m.mul_to(0, -4., 1);
    println!("=> {:#}", m);
    m.div(1, -600.);
    println!("=> {:#}", m);
    m.mul_to(1, -300., 0);
    println!("=> {:#}", m);
    m.div(0, 90_000.);
    println!("=> {:#}", m);
}
