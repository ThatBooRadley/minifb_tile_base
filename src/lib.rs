//! minifb tile base is an attempt to make an easy-to-use, 2d tile-based framework for [minifb](https://crates.io/crates/minifb).
//! Mainly, this is for personal use so I will update this semi-infrequently.

pub mod tools {
    pub mod color;
    pub mod dual_trait;
    pub mod matrix;
    pub mod timer;
    pub mod transform;
}
pub mod entity {
    pub mod animation;
    pub mod entity;
}
pub mod graphics {
    pub mod library;
    pub mod map;
    pub mod tile;
}
pub mod window;
