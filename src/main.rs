use crate::{
    exporters::excalidraw::ExcalidrawFile,
    matrix::{Matrix2x2, Matrix3x3},
};

pub mod exporters;
pub mod matrix;

fn main() {
    let mut m = Matrix2x2::new([[90_000., 300.], [360_000., 600.]], [620., 0.]);
    m.mul_to(0, -4., 1);
    m.div(1, -600.);
    m.mul_to(1, -300., 0);
    m.div(0, 90_000.);
    let mut m1 = Matrix3x3::new(
        [[6., 4., -1.], [-7., -8., -3.], [4., -2., 1.]],
        [0., 5., 22.],
    );
    m1.add_to(2, 0)
        .add_to(0, 2)
        .mul_to(2, 3., 1)
        .mul_to(0, -3.5, 1)
        .div(1, -15.)
        .mul_to(1, -2., 0)
        .div(0, 10.)
        .mul_to(0, -14., 2);
    let mut file = ExcalidrawFile::default();
    file.draw(&m, 0, 0, false);
    file.draw(&m1, 0, 120, false);
    println!("{}", serde_json::to_string(&file).unwrap());
}
