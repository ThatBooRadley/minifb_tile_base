use crate::tile::Tile;

pub trait Entity {
    fn get_position_matrix(&self) -> (usize, usize, &Tile);
    fn get_order(&self) -> &usize;
}
