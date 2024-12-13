use crate::tools::{color::Color, matrix::Matrix};

/// Tiles that make up a map or entity
pub trait Tile: Clone + Sized + Sync + Send {
    fn get_matrix(&self) -> &Matrix<Option<Color>>;
    fn get_iter(&self) -> impl Iterator<Item = Option<Color>>;
}
