use core::panic;

use crate::tools::matrix::Matrix;
use minifb::{Scale, ScaleMode, Window, WindowOptions};

pub struct WindowController {
    pub matrix: Matrix<u32>,
    pub window: Window,
}

impl WindowController {
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

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.matrix.values, self.matrix.width, self.matrix.height)
            .unwrap_or_else(|e| panic!("{}", e))
    }
}
