mod view;
use chrono::{self, Datelike, Local, Timelike};
use std::{
    fs::{create_dir, File},
    io::Write,
    path::Path,
    process::Command,
};

fn main() {
    let now = Local::now();
    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!("Work on journaling app is in progress");

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
    if !Path::exists(&file_path) {
        let mut file = File::create(&file_path).expect("Error while creating file");
        let init = format!("date = \"{}\"\nmarkdown = \"\"\"\n\"\"\"", date);
        file.write_all(init.as_bytes())
            .expect("Error while writing to file");
    } else {
        println!("Today's journal already exists")
    }

    view::start().expect("Error starting TUI");

    let mut output = Command::new("vim");
    output.arg(&file_path).status().expect("Error starting Vim");
}
