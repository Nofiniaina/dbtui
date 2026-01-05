use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::Rgb(48, 54, 61)))
        .style(Style::default().bg(Color::Rgb(33, 38, 45)));

    let widget = Paragraph::new(Text::styled(
        " DBTUI Workbench",
        Style::default()
            .fg(Color::Rgb(88, 166, 255))
            .add_modifier(Modifier::BOLD),
    ))
    .block(block);

    frame.render_widget(widget, area);
}
