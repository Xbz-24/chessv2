// src/gui/draw.rs

use crate::gui::window::ChessWindow;

pub struct Draw {
    window: ChessWindow,
    // other attributes...
}

impl Draw {
    pub fn new(window: ChessWindow) -> Draw {
        Draw { window }
    }

    pub fn draw_board(&self) {
        // draw the board...
    }

    pub fn draw_pieces(&self) {
        // draw the pieces...
    }

    pub fn highlight_moves(&self) {
        // highlight the possible moves...
    }

    // more methods...
}
