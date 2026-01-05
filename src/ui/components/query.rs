use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(48, 54, 61)))
        .style(Style::default().bg(Color::Rgb(22, 27, 34)));

    let widget =
        Paragraph::new(Text::styled("Query", Style::default().fg(Color::Green))).block(block);

    frame.render_widget(widget, area);
}
