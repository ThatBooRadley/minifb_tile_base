use crate::tools::matrix::Matrix;

use super::pixel::Pixel;

/// Tiles that make up a map or entity
pub trait Tile: Clone + Sized + Sync + Send {
    fn get_matrix(&self) -> &Matrix<Pixel>;
    fn get_iter(&self) -> impl Iterator<Item = &Pixel>;
}
