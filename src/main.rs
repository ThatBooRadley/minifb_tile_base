use minifb::{Key, Scale};
use minifb_tile_base::{tools::matrix::Matrix, window::WindowController};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut window_controller = WindowController::new("title", WIDTH, HEIGHT, Scale::X4, true);
    window_controller.matrix.values.fill(500);

    let mut matrix = Matrix::new(25, 25, false);

    while window_controller.window.is_open() && !window_controller.window.is_key_down(Key::Escape) {
        for i in 0..25 {
            for j in 0..25 {
                matrix.apply(i, j, |u| u * i as u32 + j as u32);
            }
        }
        window_controller.matrix.overlay(&matrix, 95, 95);
        window_controller.update()
    }
}
