// src/gui/window.rs

use minifb::{Window, WindowOptions, Scale, ScaleMode, Key};

pub struct ChessWindow {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl ChessWindow {
    pub fn new(width: usize, height: usize) -> ChessWindow {
        let window = Window::new(
            "Chess Game",
            width,
            height,
            WindowOptions {
                borderless: false,
                title: true,
                resize: false,
                scale: Scale::X1,
                scale_mode: ScaleMode::AspectRatioStretch,
                topmost: false,
                transparency: false,
                none: false,
            },
        ).unwrap();

        let buffer = vec![0; width * height];

        ChessWindow { window, buffer, width, height }
    }

    pub fn update(&mut self) {
        if self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
        }
    }

    // more methods...
}
