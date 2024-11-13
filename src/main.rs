use minifb::{Key, Scale};
use minifb_tile_base::{tools::matrix::Matrix, window::WindowController};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut window_controller = WindowController::new("title", WIDTH, HEIGHT, Scale::X4, true);
    window_controller.matrix.values.fill(500);

    let mut matrix = Matrix::new(5, 5, false);

    while window_controller.window.is_open() && !window_controller.window.is_key_down(Key::Escape) {
        matrix
            .enumerate_mut()
            .for_each(|(x, y, item)| *item = x as u32 + *item * y as u32);
        window_controller.matrix.overlay(&matrix, 95, 95);
        window_controller.update()
    }

    println!(
        "og: {:?}\n clamp {:?}",
        matrix.enumerate().collect::<Vec<_>>(),
        matrix.clamp(2, 2, 2, 2).collect::<Vec<_>>()
    );
}
