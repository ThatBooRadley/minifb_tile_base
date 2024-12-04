use crate::tools::matrix::Matrix;

use super::{color::ColorMap, pixel::Pixel, tile::Tile};

/// Where each tile is placed
pub struct TileMap<T: Tile> {
    /// matrix storing the tiles in the map
    pub map: Matrix<Option<T>>,
    /// width in pixels of each tile
    tile_width: usize,
    /// height in pixels of each tile
    tile_height: usize,
    /// How the tiles are displayed
    pub palatte: ColorMap,
    /// color rendition of map
    pub buffer: Matrix<u32>,
}

impl<T: Tile> TileMap<T> {
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
            palatte: ColorMap::new(),
            buffer: Matrix::new(width * tile_width, height * tile_height, wrapping),
        }
    }

    /// updates tilemap buffer. must be done at least once to have tilemap display
    pub fn update_buffer(&mut self) {
        self.map.enumerate().for_each(|(x, y, u)| {
            if let Some(tile) = u {
                self.buffer.overlay_iter(
                    tile.get_iter().map(|p| match p {
                        Pixel::Color(u) => u,
                        Pixel::Value(v) => self.palatte.get(*v).unwrap_or(&0),
                        Pixel::None => &0,
                    }),
                    x * self.tile_width,
                    y * self.tile_height,
                    self.tile_width,
                    self.tile_height,
                )
            }
        })
    }
}
