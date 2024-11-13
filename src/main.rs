use minifb::{Key, Scale};
use minifb_tile_base::window::WindowController;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut window_controller = WindowController::new("title", WIDTH, HEIGHT, Scale::X4);

    while window_controller.window.is_open() && !window_controller.window.is_key_down(Key::Escape) {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                window_controller
                    .matrix
                    .apply(i, j, |u| u * i as u32 + j as u32);
            }
        }
        window_controller.update()
    }
}
