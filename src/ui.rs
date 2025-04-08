use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style, Stylize},
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
                        .title("Chip Ate")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green)),
                )
                .marker(ratatui::symbols::Marker::Block)
                .paint(|ctx| {
                    for y in 0..32 {
                        for x in 0..64 {
                            if display[y * 64 + x] == 1 {
                                ctx.print(x as f64, (31 - y) as f64, "█".fg(Color::Green));
                            }
                        }
                    }
                })
                .x_bounds([0.0, 64.0])
                .y_bounds([0.0, 32.0]);

            let display_width = 64 + 2; // with borders
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
