use exporters::excalidraw::ExcalidrawFile;
use matrix::with_result::LineMatrix;

pub mod exporters;
pub mod matrix;

fn main() {
    const SIZE: usize = 5;
    let m = LineMatrix::new([[42.1; SIZE]; SIZE], [42.1; SIZE]);
    let mut exc = ExcalidrawFile::default();
    let (width, height) = exc.draw(&m, 0, 0, false);
    exc.draw(&m, width, height, false);
    print!("{}", serde_json::to_string(&exc).unwrap());
}
