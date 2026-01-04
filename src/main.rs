use crossterm::event::{self, DisableMouseCapture, Event, KeyCode};
use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::Terminal;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::prelude::{Backend, CrosstermBackend};
use std::error::Error;
use std::io;

mod app;
mod types;
mod ui;

use app::App;
use types::CurrentScreen;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    let _ = execute!(stderr, EnterAlternateScreen, EnableMouseCapture);

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app)?;

    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool, Box<dyn Error>>
where
    <B as Backend>::Error: 'static,
{
    loop {
        terminal.draw(|frame| match app.current_screen {
            CurrentScreen::Login => ui::login::draw(frame),
            CurrentScreen::Workbench => ui::workbench::draw(frame),
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Login => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    _ => {}
                },
                CurrentScreen::Workbench => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    _ => {}
                },
            }
        }
    }
}
