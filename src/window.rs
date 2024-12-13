use crate::{
    entity::entity::Entity,
    graphics::tile::Tile,
    tools::{
        color::Color,
        matrix::Matrix,
        transform::{Dimensions, Transform},
    },
};
use minifb::{Error, Scale, ScaleMode, Window, WindowOptions};

/// WindowController holds the main interaction between the actual matrix holding the tiles and the
/// minifb Window.

pub struct WindowController {
    /// Where colors to be displayed are stored.
    pub matrix: Matrix<Color>,
    /// The actual values that are transfered to the window
    buffer: Vec<u32>,
    /// Provided by minifb, the device that displays the tiles.
    pub window: Window,
}

impl WindowController {
    /// Creates a new WindowController with intended matrix width and height.
    /// The scale is only initial scale when the Window is created. It may be rescaled.
    /// The Window is set with target 60fps.
    pub fn new(name: &str, dimensions: Dimensions, scale: Scale, wrapping: bool) -> Self {
        let mut window = Window::new(
            name,
            dimensions.width,
            dimensions.height,
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
            matrix: Matrix::new(dimensions, wrapping),
            buffer: vec![0; dimensions.area()],
            window,
        }
    }

    /// Update Window buffer every frame wanted.
    pub fn update(&mut self) -> Result<(), Error> {
        self.window.update_with_buffer(
            &self.buffer,
            self.matrix.dimensions.width,
            self.matrix.dimensions.height,
        )
    }

    pub fn update_buffer(&mut self, buffer: impl Iterator<Item = Color>) -> Result<(), Error> {
        self.buffer = buffer.map(|c| u32::from(c)).collect::<Vec<_>>();
        self.update()
    }

    /// Updates window buffer each frame called and adds entities.
    pub fn update_with_entities(&mut self, entities: &mut [impl Entity]) -> Result<(), Error> {
        let mut matrix_with_entities = self.matrix.clone();
        entities.sort_by(|a, b| a.get_order().cmp(&b.get_order()));

        entities.iter_mut().for_each(|e| {
            let (Transform { position, rotation }, tile) = e.get_position_matrix();
            let tile_matrix = tile.get_matrix();
            matrix_with_entities.transparent_overlay_iter(
                tile_matrix.iter_rotate(*rotation).copied(),
                *position,
                tile_matrix.dimensions,
            )
        });

        self.update_buffer(matrix_with_entities.values.iter().copied())
    }
}
