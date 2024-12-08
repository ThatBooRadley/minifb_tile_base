use core::panic;

use crate::{
    entity::entity::Entity,
    graphics::{pixel::Pixel, tile::Tile},
    tools::{
        matrix::Matrix,
        transform::{Size, Transform},
    },
};
use minifb::{Scale, ScaleMode, Window, WindowOptions};

/// WindowController holds the main interaction between the actual matrix holding the tiles and the
/// minifb Window.

pub struct WindowController {
    /// Where tiles to be displayed are stored.
    pub matrix: Matrix<u32>,
    /// Provided by minifb, the device that displays the tiles.
    pub window: Window,
}

impl WindowController {
    /// Creates a new WindowController with intended matrix width and height.
    /// The scale is only initial scale when the Window is created. It may be rescaled.
    /// The Window is set with target 60fps.
    pub fn new(name: &str, size: Size, scale: Scale, wrapping: bool) -> Self {
        let mut window = Window::new(
            name,
            size.width,
            size.height,
            WindowOptions {
                scale_mode: ScaleMode::AspectRatioStretch,
                scale,
                resize: true,
                borderless: false,
                ..Default::default()
            },
        )
        .unwrap();
        window.set_target_fps(60);

        Self {
            matrix: Matrix::new(size, wrapping),
            window,
        }
    }

    /// Update Window buffer every frame wanted.
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(
                &self.matrix.values,
                self.matrix.size.width,
                self.matrix.size.height,
            )
            .unwrap_or_else(|e| panic!("{}", e))
    }

    /// Updates window buffer each frame called and adds entities.
    pub fn update_with_entities(&mut self, entities: &mut [impl Entity]) {
        let mut matrix_with_entities = self.matrix.clone();
        entities.sort_by(|a, b| a.get_order().cmp(&b.get_order()));

        entities.iter_mut().for_each(|e| {
            let (Transform { position, rotation }, tile) = e.get_position_matrix();
            let tile_matrix = tile.get_matrix();
            matrix_with_entities.transparent_overlay_iter(
                tile_matrix.iter_rotate(*rotation).map(|p| {
                    if let Pixel::Color(u) = p {
                        Some(*u)
                    } else {
                        None
                    }
                }),
                *position,
                tile_matrix.size,
            )
        });

        self.window
            .update_with_buffer(
                &matrix_with_entities.values,
                matrix_with_entities.size.width,
                matrix_with_entities.size.height,
            )
            .unwrap_or_else(|e| panic!("{}", e))
    }
}
