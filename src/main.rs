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
    widgets::block::Block,
    widgets::{BorderType, Borders, Paragraph},
    Frame,
};
use std::io::{stdout, Result};
use std::process::Command;
// use tui_textarea::TextArea;
mod app_state;
// mod editor;
// use editor::{Mode, Transition, Vim};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // vim stuff
    // let mut text_area = if let Some(path) = env::args().nth(1) {
    //     let file = std::fs::File::open(path)?;
    //     io::BufReader::new(file)
    //         .lines()
    //         .collect::<io::Result<_>>()?
    // } else {
    //     TextArea::default()
    // };

    // text_area.set_block(Mode::Normal.block());
    // text_area.set_cursor_style(Mode::Normal.cursor_style());
    // let mut vim = Vim::new(Mode::Normal);
    // ========

    let mut app_state = app_state::AppState::new();

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
            ui(frame, &app_state);
            // frame.render_widget(text_area.widget(), frame.size());
        })?;

        // vim = match vim.transition(crossterm::event::read()?.into(), &mut text_area) {
        //     Transition::Mode(mode) if vim.mode != mode => {
        //         text_area.set_block(mode.block());
        //         text_area.set_cursor_style(mode.cursor_style());
        //         Vim::new(mode)
        //     }
        //     Transition::Nop | Transition::Mode(_) => vim,
        //     Transition::Pending(input) => vim.with_pending(input),
        //     Transition::Quit => break,
        // };
        // TODO handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            let event = crossterm::event::read()?;
            app_state.input(&event);
            if let event::Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                    if key.code == KeyCode::Char('w') {
                        let test_arg = format!(
                            "tell app \"Terminal\" to do script \"vim ~/Development/log/{0}\"\nreturn",
                            app_state.path.to_str().unwrap()
                        );
                        let mut output = Command::new("osascript");
                        output.arg("-e").arg(test_arg).status().expect("Error");
                        // let mut output = Command::new("vim");
                        // output
                        //     .arg(&app_state.path)
                        //     .status()
                        //     .expect("Error starting Vim");
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(frame: &mut Frame, app_state: &AppState) {
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
        [
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ],
    )
    .split(main_layout[1]);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Entries")
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::LightBlue));

    let paragraph = app_state.render();
    frame.render_widget(paragraph.block(block), inner_layout[0]);
    frame.render_widget(app_state.test(), inner_layout[1]);
    frame.render_widget(app_state.test(), inner_layout[2]);
}
