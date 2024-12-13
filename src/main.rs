use minifb::{Key, Scale};
use minifb_tile_base::{
    entity::entity::Entity,
    graphics::{map::TileMap, pixel::Pixel, tile::Tile},
    tools::{
        dual_trait::Algebra,
        matrix::Matrix,
        transform::{Dimensions, Position, Rotation, Transform},
    },
    window::WindowController,
};

const DIMENSIONS: Dimensions = Dimensions {
    width: 100,
    height: 100,
};

#[derive(Clone)]
struct Player {
    transform: Transform,
    matrix: Matrix<Pixel>,
}

impl Tile for Player {
    fn get_iter(&self) -> impl Iterator<Item = &Pixel> {
        self.matrix.values.iter()
    }

    fn get_matrix(&self) -> &Matrix<Pixel> {
        &self.matrix
    }
}

impl Entity for Player {
    fn get_order(&self) -> &usize {
        &0
    }
    fn get_position_matrix(&self) -> (&Transform, &impl Tile) {
        (&self.transform, self)
    }
}

#[derive(Clone)]
enum TileBase {
    ONE(Matrix<Pixel>),
    TWO(Matrix<Pixel>),
    THREE(Matrix<Pixel>),
    FOUR(Matrix<Pixel>),
    FIVE(Matrix<Pixel>),
}

impl TileBase {
    fn from_usize(i: usize, matrix: Matrix<Pixel>) -> Self {
        match i % 5 {
            0 => Self::ONE(matrix),
            1 => Self::TWO(matrix),
            2 => Self::THREE(matrix),
            3 => Self::FOUR(matrix),
            _ => Self::FIVE(matrix),
        }
    }
}

impl Tile for TileBase {
    fn get_iter(&self) -> impl Iterator<Item = &Pixel> {
        self.get_matrix().values.iter()
    }

    fn get_matrix(&self) -> &Matrix<Pixel> {
        match self {
            Self::ONE(m) | Self::TWO(m) | Self::THREE(m) | Self::FOUR(m) | Self::FIVE(m) => m,
        }
    }
}

fn main() {
    let mut window_controller = WindowController::new("title", DIMENSIONS, Scale::X4, true);
    window_controller.matrix.values.fill(500.into());

    let mut player = Player {
        transform: Transform::default(),
        matrix: Matrix {
            dimensions: Dimensions::splat(4),
            values: vec![
                Pixel::None,
                Pixel::Color(0xFF0000.into()),
                Pixel::Color(0xFF0000.into()),
                Pixel::None,
                Pixel::Color(0xFFFFFF.into()),
                Pixel::Color(0xFF0000.into()),
                Pixel::Color(0xFF0000.into()),
                Pixel::Color(0xFFFFFF.into()),
                Pixel::Color(0xFF0000.into()),
                Pixel::Color(0xFF0000.into()),
                Pixel::Color(0xFF0000.into()),
                Pixel::Color(0xFF0000.into()),
                Pixel::None,
                Pixel::Color(0xFF0000.into()),
                Pixel::Color(0xFF0000.into()),
                Pixel::None,
            ],
            wrapping: false,
        },
    };

    let mut map = TileMap::<TileBase>::new(Dimensions::splat(25), false, Dimensions::splat(4));
    /*
        map.map.enumerate_mut().for_each(|(position, u)| {
            *u = Some(match position.mul_self() % 5 {
                0 => TileBase::from_usize(
                    0,
                    Matrix {
                        values: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
                            .iter()
                            .map(|u| Pixel::Value(*u))
                            .collect::<Vec<_>>(),
                        dimensions: Dimensions::splat(4),
                        wrapping: false,
                    },
                ),
                1 => TileBase::from_usize(
                    1,
                    Matrix {
                        values: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
                            .iter()
                            .rev()
                            .map(|u| Pixel::Value(*u))
                            .collect::<Vec<_>>(),

                        dimensions: Dimensions::splat(4),
                        wrapping: false,
                    },
                ),
                2 => TileBase::from_usize(
                    2,
                    Matrix {
                        values: [0, 2, 1, 3, 4, 6, 5, 7, 8, 10, 9, 11, 12, 14, 13, 15]
                            .iter()
                            .map(|u| Pixel::Value(*u))
                            .collect::<Vec<_>>(),

                        dimensions: Dimensions::splat(4),
                        wrapping: false,
                    },
                ),
                3 => TileBase::from_usize(
                    3,
                    Matrix {
                        values: [0, 2, 1, 3, 4, 6, 5, 7, 8, 10, 9, 11, 12, 14, 13, 15]
                            .iter()
                            .rev()
                            .map(|u| Pixel::Value(*u))
                            .collect::<Vec<_>>(),

                        dimensions: Dimensions::splat(4),
                        wrapping: false,
                    },
                ),
                _ => TileBase::from_usize(
                    4,
                    Matrix {
                        values: [3, 1, 2, 0, 7, 5, 6, 4, 11, 9, 10, 8, 15, 13, 14, 12]
                            .iter()
                            .map(|u| Pixel::Value(*u))
                            .collect::<Vec<_>>(),

                        dimensions: Dimensions::splat(4),
                        wrapping: false,
                    },
                ),
            })
        });

        (0..16).for_each(|i| {
            map.palatte.add(i, 0xF << i);
            println!("None {}, {:?}", i, map.palatte.get(i))
        });
    */
    map.update_buffer();

    while window_controller.window.is_open() && !window_controller.window.is_key_down(Key::Escape) {
        window_controller
            .window
            .get_keys()
            .iter()
            .for_each(|k| match k {
                Key::W => {
                    player.transform.position.y -= 1;
                    player.transform.rotation = Rotation::UP
                }
                Key::A => {
                    player.transform.position.x -= 1;
                    player.transform.rotation = Rotation::LEFT
                }
                Key::S => {
                    player.transform.position.y += 1;
                    player.transform.rotation = Rotation::DOWN
                }
                Key::D => {
                    player.transform.position.x += 1;
                    player.transform.rotation = Rotation::RIGHT
                }
                _ => (),
            });

        window_controller
            .matrix
            .overlay(&map.buffer, Position::splat(0));
        window_controller.update_with_entities(&mut [player.clone()])
    }
}
