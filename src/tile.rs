use crate::tools::matrix::Matrix;

/// An individual tile
#[derive(Clone)]
pub enum Tile {
    /// Every pixel has a value
    Simple(Matrix<u32>),
    /// Only pixels with Some have a value
    Transparent(Matrix<Option<u32>>),
}

impl Tile {
    pub fn new_simple(width: usize, height: usize) -> Self {
        Self::Simple(Matrix::new(width, height, false))
    }

    pub fn new_transparent(width: usize, height: usize) -> Self {
        Self::Transparent(Matrix::new(width, height, false))
    }

    pub fn to_simple(&self) -> Self {
        match self {
            Self::Transparent(matrix) => Self::Simple(Matrix {
                values: matrix
                    .values
                    .iter()
                    .map(|u| u.unwrap_or_default())
                    .collect::<Vec<u32>>(),
                width: matrix.width,
                height: matrix.height,
                wrapping: false,
            }),
            _ => self.clone(),
        }
    }
    pub fn to_transparent(&self) -> Self {
        match self {
            Self::Simple(matrix) => Self::Transparent(Matrix {
                values: matrix
                    .values
                    .iter()
                    .map(|&u| Some(u))
                    .collect::<Vec<Option<u32>>>(),
                width: matrix.width,
                height: matrix.height,
                wrapping: false,
            }),
            _ => self.clone(),
        }
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

    pub fn update_buffer(&mut self, tile_library: &TileLibrary) {
        self.map.enumerate().for_each(|(x, y, u)| {
            if let Some(index) = u {
                if let Some(tile) = tile_library.get(*index) {
                    match tile {
                        Tile::Simple(m) => {
                            self.buffer
                                .overlay(&m, x * self.tile_width, y * self.tile_height)
                        }
                        Tile::Transparent(m) => self.buffer.transparent_overlay(
                            &m,
                            x * self.tile_width,
                            y * self.tile_height,
                        ),
                    }
                }
            }
        })
    }
}
