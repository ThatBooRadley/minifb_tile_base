use crate::{graphics::tile::Tile, tools::transform::Transform};

pub trait Entity {
    fn get_position_matrix(&self) -> (&Transform, &impl Tile);
    fn get_order(&self) -> &usize;
}
