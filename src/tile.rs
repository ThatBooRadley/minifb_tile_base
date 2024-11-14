use crate::tools::matrix::Matrix;

pub struct Tile(pub Matrix<u32>);

/// An individual tile
impl Tile {
    pub fn new(width: usize, height: usize) -> Self {
        Self(Matrix::new(width, height, false))
    }
}

/// Where each type of tile is stored
pub struct TileLibrary(Vec<Tile>);

impl TileLibrary {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, tile: Tile) {
        self.0.push(tile)
    }

    pub fn get(&self, index: usize) -> Option<&Tile> {
        self.0.get(index)
    }
}

/// Where each tile is placed
pub struct TileMap {
    /// matrix storing the tiles in the map
    pub map: Matrix<Option<usize>>,
    /// width in pixels of each tile
    tile_width: usize,
    /// height in pixels of each tile
    tile_height: usize,
    /// pixel rendition of map
    pub buffer: Matrix<u32>,
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
            buffer: Matrix::new(width * tile_width, height * tile_height, wrapping),
        }
    }

    pub fn update_buffer(&mut self, tile_library: TileLibrary) {
        self.map.enumerate().for_each(|(x, y, u)| {
            if let Some(index) = u {
                if let Some(tile) = tile_library.get(*index) {
                    self.buffer
                        .overlay(&tile.0, x * self.tile_width, y * self.tile_height)
                }
            }
        })
    }
}
