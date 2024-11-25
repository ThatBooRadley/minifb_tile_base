use minifb::{Key, Scale};
use minifb_tile_base::{
    entity::entity::Entity,
    tile::{Tile, TileLibrary, TileMap},
    tools::{
        matrix::Matrix,
        transform::{Rotation, Transform},
    },
    window::WindowController,
};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

#[derive(Clone)]
struct Player {
    transform: Transform,
    tile: Tile,
}

impl Entity for Player {
    fn get_order(&self) -> &usize {
        &0
    }
    fn get_position_matrix(&self) -> (&Transform, &Tile) {
        (&self.transform, &self.tile)
    }
}

fn main() {
    let mut window_controller = WindowController::new("title", WIDTH, HEIGHT, Scale::X4, true);
    window_controller.matrix.values.fill(500);

    let mut player = Player {
        transform: Transform::default(),
        tile: Tile::Transparent(Matrix {
            width: 4,
            height: 4,
            values: vec![
                None,
                Some(0xFF0000),
                Some(0xFF0000),
                None,
                Some(0xFFFFFF),
                Some(0xFF0000),
                Some(0xFF0000),
                Some(0xFFFFFF),
                Some(0xFF0000),
                Some(0xFF0000),
                Some(0xFF0000),
                Some(0xFF0000),
                None,
                Some(0xFF0000),
                Some(0xFF0000),
                None,
            ],
            wrapping: false,
        }),
    };

    let mut library = TileLibrary::new();
    for i in 0..25 {
        let mut matrix = Matrix::new(4, 4, false);
        matrix
            .enumerate_mut()
            .for_each(|(x, y, u)| *u = x as u32 * y as u32 * 7 * i);
        library.add(Tile::Simple(matrix));
    }

    let mut map = TileMap::new(25, 25, false, 4, 4);
    map.map
        .enumerate_mut()
        .for_each(|(x, y, u)| *u = Some((x * y) % 25));
    map.update_buffer(&library);

    let mut trans_matrix = Matrix::new(5, 5, false);
    (0..5).for_each(|i| trans_matrix.set(i, i, Some(0xFFFFFF)));

    while window_controller.window.is_open() && !window_controller.window.is_key_down(Key::Escape) {
        window_controller
            .window
            .get_keys()
            .iter()
            .for_each(|k| match k {
                Key::W => {
                    player.transform.y -= 1;
                    player.transform.rotation = Rotation::UP
                }
                Key::A => {
                    player.transform.x -= 1;
                    player.transform.rotation = Rotation::LEFT
                }
                Key::S => {
                    player.transform.y += 1;
                    player.transform.rotation = Rotation::DOWN
                }
                Key::D => {
                    player.transform.x += 1;
                    player.transform.rotation = Rotation::RIGHT
                }
                _ => (),
            });

        window_controller.matrix.overlay(&map.buffer, 0, 0);
        window_controller
            .matrix
            .transparent_overlay(&trans_matrix, 2, 2);
        window_controller.update_with_entities(&mut [player.clone()])
    }
}
