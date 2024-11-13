use core::panic;

use crate::tools::matrix::Matrix;
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
    pub fn new(name: &str, width: usize, height: usize, scale: Scale) -> Self {
        let mut window = Window::new(
            name,
            width,
            height,
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
            matrix: Matrix::new(width, height),
            window,
        }
    }

    /// Update Window buffer every frame wanted.
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.matrix.values, self.matrix.width, self.matrix.height)
            .unwrap_or_else(|e| panic!("{}", e))
    }
}
