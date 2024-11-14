use crate::{tile::Tile, tools::matrix::Matrix};

/// Tile drawn above tilemap
#[derive(Clone)]
pub struct Entity {
    /// x position of pixels
    pub x: usize,
    /// y position of pixels
    pub y: usize,
    /// Image rendered by entity.
    pub tile: Tile,
    /// order in which entities are drawn if overlapped (lowest first).
    pub order: usize,
}

impl Entity {
    /// used for overlay
    pub fn to_position_matrix(&self) -> (usize, usize, &Matrix<u32>) {
        (self.x, self.y, &self.tile.0)
    }
}
