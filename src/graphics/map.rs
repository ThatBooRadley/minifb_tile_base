use crate::tools::{
    color::Color,
    dual_trait::Algebra,
    matrix::Matrix,
    transform::{Dimensions, Position},
};

use super::{pixel::Pixel, tile::Tile};

/// Where each tile is placed
pub struct TileMap<T: Tile> {
    /// matrix storing the tiles in the map
    pub map: Matrix<Option<T>>,
    /// dimensions in pixels of each tile
    tile_dimensions: Dimensions,
    /// color rendition of map
    pub buffer: Matrix<Color>,
}

impl<T: Tile> TileMap<T> {
    pub fn new(dimensions: Dimensions, wrapping: bool, tile_dimensions: Dimensions) -> Self {
        Self {
            map: Matrix::new(dimensions, wrapping),
            tile_dimensions,
            buffer: Matrix::new(dimensions.mul(tile_dimensions), wrapping),
        }
    }

    /// updates tilemap buffer. must be done at least once to have tilemap display
    pub fn update_buffer(&mut self) {
        self.map.enumerate().for_each(|(position, u)| {
            if let Some(tile) = u {
                self.buffer.transparent_overlay_iter(
                    tile.get_iter().map(|p| match p {
                        Pixel::Color(u) => Some(*u),
                        Pixel::None => None,
                    }),
                    position.mul(self.tile_dimensions.into_dual::<Position>()),
                    self.tile_dimensions,
                )
            }
        })
    }
}
