use crate::tools::matrix::Matrix;
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use rayon::iter::ParallelIterator;

pub struct WindowController {
    pub real_window: Matrix<u32>,
    virtual_window: Matrix<u32>,
    ratio: usize,
    pub window: Window,
}

impl WindowController {
    pub fn new(name: &str, width: usize, height: usize, ratio: usize) -> Self {
        //width and height correspond to real window size
        let mut window = Window::new(
            name,
            width * ratio,
            height * ratio,
            WindowOptions {
                scale_mode: ScaleMode::Center,
                scale: Scale::FitScreen,
                resize: true,
                ..Default::default()
            },
        )
        .unwrap_or_else(|e| panic!("{}", e));
        window.set_target_fps(60);

        Self {
            real_window: Matrix::new(width, height),
            virtual_window: Matrix::new(width * ratio, height * ratio),
            ratio,
            window,
        }
    }

    pub fn project(&mut self) {
        self.virtual_window
            .enumerate_mut()
            .for_each(|(x, y, item)| {
                if let Some(&u) = self.real_window.get(x / self.ratio, y / self.ratio) {
                    *item = u;
                }
            })
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(
                &self.virtual_window.values,
                self.virtual_window.width,
                self.virtual_window.height,
            )
            .unwrap();
    }

    pub fn project_update(&mut self) {
        self.project();
        self.update();
    }

    pub fn is_resized(&self) -> bool {
        let size = self.window.get_size();
        if size.0 != self.virtual_window.width || size.1 != self.virtual_window.height {
            true
        } else {
            false
        }
    }
}
