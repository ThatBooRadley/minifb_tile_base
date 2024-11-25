use crate::{tile::Tile, tools::transform::Transform};

pub trait Entity {
    fn get_position_matrix(&mut self) -> (&Transform, &Tile);
    fn get_order(&self) -> &usize;
}
