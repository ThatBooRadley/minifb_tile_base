use minifb::Key;
use minifb_tile_base::window::WindowController;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const RATIO: usize = 8;

fn main() {
    let mut window_controller = WindowController::new("title", WIDTH, HEIGHT, RATIO);

    while window_controller.window.is_open() && !window_controller.window.is_key_down(Key::Escape) {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                window_controller
                    .real_window
                    .apply(i, j, |u| u * i as u32 + j as u32);
            }
        }
        window_controller.project_update()
    }
}
