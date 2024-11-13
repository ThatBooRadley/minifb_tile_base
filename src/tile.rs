use crate::tools::matrix::Matrix;

pub struct Tile(Matrix<u32>);

/// An individual tile
impl Tile {
    pub fn new(width: usize, height: usize) -> Self {
        Self(Matrix::new(width, height, false))
    }
}

/// Where each type of tile is stored
pub struct TileLibrary<'a>(Vec<&'a Tile>);

impl<'a> TileLibrary<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, tile: &'a Tile) {
        self.0.push(tile)
    }

    pub fn get(&self, index: usize) -> Option<&&Tile> {
        self.0.get(index)
    }
}

/// Where each tile is placed
pub struct TileMap {
    map: Matrix<usize>,
    /// width in pixels of each tile
    tile_width: usize,
    /// height in pixels of each tile
    tile_height: usize,
}

impl TileMap {
    pub fn new(
        width: usize,
        height: usize,
        wrapping: bool,
        tile_width: usize,
        tile_height: usize,
    ) -> Self {
        Self {
            map: Matrix::new(width, height, wrapping),
            tile_width,
            tile_height,
        }
    }
}
