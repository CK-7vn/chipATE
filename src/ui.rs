//// src/ui.rs
//use ratatui::{
//    backend::Backend,
//    widgets::{Block, Borders, Padding, Paragraph},
//    Terminal,
//};
//use std::io;
//
//pub struct UI<B: Backend> {
//    pub terminal: Terminal<B>,
//}
//
//impl<B: Backend> UI<B> {
//    pub fn new(terminal: Terminal<B>) -> Self {
//        UI { terminal }
//    }
//
//    pub fn render(&mut self, display_text: &str) -> Result<(), io::Error> {
//        self.terminal.draw(|frame| {
//            let widget = Paragraph::new(display_text).block(
//                Block::default()
//                    .title("Chip-8 Emulator")
//                    .padding(Padding::ZERO)
//                    .borders(Borders::ALL),
//            );
//            frame.render_widget(widget, frame.area());
//        })?;
//        Ok(())
//    }
//
//    pub fn cleanup(&mut self) -> Result<(), io::Error> {
//        self.terminal.clear()?;
//        Ok(())
//    }
//}
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders},
    Terminal,
};
use std::io;

pub struct UI<B: Backend> {
    pub terminal: Terminal<B>,
}

impl<B: Backend> UI<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        UI { terminal }
    }

    pub fn render(&mut self, display: &[u8; 64 * 32]) -> Result<(), io::Error> {
        self.terminal.draw(|frame| {
            let size = frame.area();

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .title("Chip-8 Emulator")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green)),
                )
                .paint(|ctx| {
                    for y in 0..32 {
                        for x in 0..64 {
                            if display[y * 64 + x] == 1 {
                                ctx.draw(&ratatui::widgets::canvas::Rectangle {
                                    x: x as f64,
                                    y: (31 - y) as f64,
                                    width: 1.5,
                                    height: 1.0,
                                    color: Color::LightGreen,
                                });
                            }
                        }
                    }
                })
                .x_bounds([0.0, 64.0])
                .y_bounds([0.0, 32.0]);

            // Center the canvas
            let display_width = 64 + 2;
            let display_height = 32 + 2;
            let x_offset = (size.width.saturating_sub(display_width)) / 2;
            let y_offset = (size.height.saturating_sub(display_height)) / 2;
            let display_area = Rect::new(x_offset, y_offset, display_width, display_height);

            frame.render_widget(canvas, display_area);
        })?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), io::Error> {
        self.terminal.clear()?;
        Ok(())
    }
}
