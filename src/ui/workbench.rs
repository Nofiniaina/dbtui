use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use super::components::{header, sidebar, query, results, footer};

pub fn draw(frame: &mut Frame) {
    // root layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // header
    header::render(frame, chunks[0]);

    // main layout
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    // sidebar
    sidebar::render(frame, main_layout[0]);

    // right layout
    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    // query
    query::render(frame, right_layout[0]);

    // results
    results::render(frame, right_layout[1]);

    // footer
    footer::render(frame, chunks[2]);
}
