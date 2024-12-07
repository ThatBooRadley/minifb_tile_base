use crate::tools::{
    dual_trait::Algebra,
    matrix::Matrix,
    transform::{Position, Size},
};

use super::{color_map::ColorMap, pixel::Pixel, tile::Tile};

/// Where each tile is placed
pub struct TileMap<T: Tile> {
    /// matrix storing the tiles in the map
    pub map: Matrix<Option<T>>,
    /// size in pixels of each tile
    tile_size: Size,
    /// How the tiles are displayed
    pub palatte: ColorMap,
    /// color rendition of map
    pub buffer: Matrix<u32>,
}

impl<T: Tile> TileMap<T> {
    pub fn new(size: Size, wrapping: bool, tile_size: Size) -> Self {
        Self {
            map: Matrix::new(size, wrapping),
            tile_size,
            palatte: ColorMap::new(),
            buffer: Matrix::new(size.mul(tile_size), wrapping),
        }
    }

    /// updates tilemap buffer. must be done at least once to have tilemap display
    pub fn update_buffer(&mut self) {
        self.map.enumerate().for_each(|(position, u)| {
            if let Some(tile) = u {
                self.buffer.overlay_iter(
                    tile.get_iter().map(|p| match p {
                        Pixel::Color(u) => u,
                        Pixel::Value(v) => self.palatte.get(*v).unwrap_or(&0),
                        Pixel::None => &0,
                    }),
                    position.mul(self.tile_size.into_dual::<Position>()),
                    self.tile_size,
                )
            }
        })
    }
}
