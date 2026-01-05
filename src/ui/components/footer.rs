use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: Rect) {
    // Database conn variable
    let connection_info = "root@localhost:3306";
    let database_name = "test_db";

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::Rgb(48, 54, 61)))
        .style(Style::default().bg(Color::Rgb(22, 27, 34)));

    let status = Line::from(vec![
        Span::styled(
            " ● ",
            Style::default().fg(Color::Rgb(63, 185, 80)), // green dot for connected
        ),
        Span::styled(
            connection_info,
            Style::default().fg(Color::Rgb(110, 118, 129)), // muted text
        ),
        Span::styled(" │ ", Style::default().fg(Color::Rgb(110, 118, 129))),
        Span::styled(
            database_name,
            Style::default().fg(Color::Rgb(110, 118, 129)),
        ),
        Span::styled(" │ ", Style::default().fg(Color::Rgb(110, 118, 129))),
        Span::styled(
            "[F5] Execute | [Tab] Switch | [Ctrl+Q] Quit",
            Style::default().fg(Color::Rgb(110, 118, 129)),
        ),
    ]);

    let widget = Paragraph::new(Text::from(status)).block(block);

    frame.render_widget(widget, area);
}
