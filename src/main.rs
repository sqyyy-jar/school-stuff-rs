use crate::{
    exporters::excalidraw::{Element, ExcalidrawFile},
    matrix::Matrix2x2,
};

pub mod exporters;
pub mod matrix;

fn draw_opening_bracket(file: &mut ExcalidrawFile, x: i32, y: i32, height: i32, locked: bool) {
    file.elements.push(Element::draw_line(
        x,
        y,
        locked,
        vec![
            [file.app_state.grid_size, 0],
            [0, 0],
            [0, height * file.app_state.grid_size],
            [file.app_state.grid_size, height * file.app_state.grid_size],
        ],
    ));
}

fn draw_closing_bracket(file: &mut ExcalidrawFile, x: i32, y: i32, height: i32, locked: bool) {
    file.elements.push(Element::draw_line(
        x,
        y,
        locked,
        vec![
            [-file.app_state.grid_size, 0],
            [0, 0],
            [0, height * file.app_state.grid_size],
            [-file.app_state.grid_size, height * file.app_state.grid_size],
        ],
    ));
}

fn main() {
    let mut m = Matrix2x2::new([[90_000., 300.], [360_000., 600.]], [620., 0.]);
    println!("m = {:#}", m);
    m.mul_to(0, -4., 1);
    println!("=> {:#}", m);
    m.div(1, -600.);
    println!("=> {:#}", m);
    m.mul_to(1, -300., 0);
    println!("=> {:#}", m);
    m.div(0, 90_000.);
    println!("=> {:#}", m);
    let mut file = ExcalidrawFile::default();
    draw_opening_bracket(&mut file, 0, 0, 10, false);
    draw_closing_bracket(&mut file, 80, 0, 10, false);
    println!("{}", file.to_string());
}
