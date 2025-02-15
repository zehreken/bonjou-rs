use chrono::{self, Datelike, Local};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{List, ListDirection, Paragraph},
};
use std::{
    fs::{create_dir, File},
    io::Write,
    path::{Path, PathBuf},
};

pub struct AppState {
    pub current_path: PathBuf,
    pub path: PathBuf,
    pub date: String,
    pub year: u8,
    pub month: u8,
    pub day: u8,
}

impl AppState {
    pub fn new() -> AppState {
        let now = Local::now();
        // let (is_pm, hour) = now.hour12();

        let year_folder_name = format!("{:04}", now.year());
        let year_folder_path = Path::new(&year_folder_name);
        if !Path::exists(&year_folder_path) {
            create_dir(&year_folder_path).expect("Error while creating year folder");
        }

        let month_folder_name = format!("{:02}", now.month());
        let month_folder_path = year_folder_path.join(&month_folder_name);
        if !Path::exists(&month_folder_path) {
            create_dir(&month_folder_path).expect("Error while creating month folder");
        }

        let date = format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day());
        let file_name = format!("{}.toml", date);
        let file_path = month_folder_path.join(&file_name);
        let mut file;
        if !Path::exists(&file_path) {
            file = File::create(&file_path).expect("Error while creating file");
            let init = format!("date = \"{}\"\nmarkdown = \"\"\"\n\"\"\"", date);
            file.write_all(init.as_bytes())
                .expect("Error while writing to file");
        }

        // let mut output = Command::new("vim");
        // output.arg(&file_path).status().expect("Error starting Vim")

        AppState {
            current_path: year_folder_path.to_path_buf(),
            path: file_path,
            date,
            year: now.year() as u8,
            month: now.month() as u8,
            day: now.day() as u8,
        }
    }

    pub fn input(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('k') {
                println!("up")
            }
            if key.code == KeyCode::Char('j') {
                println!("down")
            }
            if key.code == KeyCode::Enter {}
        }
    }

    pub fn list(&self) -> List {
        let items = ["Item 1", "Item 2", "Item 3"];
        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::BottomToTop);

        list
    }

    pub fn test(&self) -> Paragraph {
        let span1 = Span::raw("Hello ");
        let span2 = Span::styled(
            "World",
            Style::new()
                .fg(Color::Green)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
        let span3 = "!".red().on_light_yellow().italic();

        let line = Line::from(vec![span1, span2, span3]);
        let text: Text = Text::from(vec![line]);
        Paragraph::new(text)
    }

    pub fn render(&self) -> Paragraph {
        Paragraph::new(String::from(self.path.to_str().unwrap()))
    }
}
