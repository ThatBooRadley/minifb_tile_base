use minifb::{Key, Scale};
use minifb_tile_base::{
    tile::{Tile, TileLibrary, TileMap},
    tools::matrix::Matrix,
    window::WindowController,
};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut window_controller = WindowController::new("title", WIDTH, HEIGHT, Scale::X4, true);
    window_controller.matrix.values.fill(500);

    let mut library = TileLibrary::new();
    for i in 0..25 {
        let mut matrix = Matrix::new(4, 4, false);
        matrix
            .enumerate_mut()
            .for_each(|(x, y, u)| *u = x as u32 * y as u32 % 7 * i);
        library.add(Tile(matrix));
    }

    let mut map = TileMap::new(25, 25, false, 4, 4);
    map.map
        .enumerate_mut()
        .for_each(|(x, y, u)| *u = Some((x * y) % 25));
    map.update_buffer(library);

    while window_controller.window.is_open() && !window_controller.window.is_key_down(Key::Escape) {
        window_controller.matrix.overlay(&map.buffer, 0, 0);
        window_controller.update()
    }
}
