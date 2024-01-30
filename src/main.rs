use app_state::AppState;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::io::BufRead;
use std::{
    env,
    io::{self, stdout, Result},
};
use tui_textarea::TextArea;
mod app_state;
mod editor;
use editor::{Mode, Transition, Vim};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // vim stuff
    let mut text_area = if let Some(path) = env::args().nth(1) {
        let file = std::fs::File::open(path)?;
        io::BufReader::new(file)
            .lines()
            .collect::<io::Result<_>>()?
    } else {
        TextArea::default()
    };

    text_area.set_block(Mode::Normal.block());
    text_area.set_cursor_style(Mode::Normal.cursor_style());
    let mut vim = Vim::new(Mode::Normal);
    // ========

    let app_state = app_state::check();

    loop {
        // TODO draw the UI
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(format!("{}\n(press SPACE to continue)", "info"))
                    .white()
                    .on_dark_gray(),
                area,
            );
            ui(frame, &app_state, &text_area);
            // frame.render_widget(text_area.widget(), frame.size());
        })?;

        vim = match vim.transition(crossterm::event::read()?.into(), &mut text_area) {
            Transition::Mode(mode) if vim.mode != mode => {
                text_area.set_block(mode.block());
                text_area.set_cursor_style(mode.cursor_style());
                Vim::new(mode)
            }
            Transition::Nop | Transition::Mode(_) => vim,
            Transition::Pending(input) => vim.with_pending(input),
            Transition::Quit => break,
        };
        // TODO handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char(' ') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(frame: &mut Frame, app_state: &AppState, text_area: &TextArea) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title("Bonjou-rs")
            .border_style(Style::default().fg(Color::Black))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Yellow)),
        main_layout[0],
    );
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(&app_state.date[..])
            .border_style(Style::default().fg(Color::Black))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Yellow)),
        main_layout[2],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(30), Constraint::Percentage(70)],
    )
    .split(main_layout[1]);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Entries")
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::LightBlue));

    let para = Paragraph::new(String::from(app_state.path.to_str().unwrap()));
    frame.render_widget(para.block(block), inner_layout[0]);
    frame.render_widget(text_area.widget(), inner_layout[1]);
}
