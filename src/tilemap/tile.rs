use crate::tools::matrix::Matrix;

#[derive(Default, Clone)]
pub struct Tile {
    /// Image of tile in Matrix
    matrix: Matrix<u32>,
}

impl Tile {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            matrix: Matrix::new(width, height),
        }
    }

    pub fn get_width(&self) -> usize {
        self.matrix.width
    }

    pub fn get_height(&self) -> usize {
        self.matrix.height
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&u32> {
        self.matrix.get(x, y)
    }
}

struct TileMap {
    map: Matrix<Tile>,
    /// Area of pixels
    pixels_per_tile: usize,
}
