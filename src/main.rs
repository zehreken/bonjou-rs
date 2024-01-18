use chrono::{self, DateTime, Datelike, Local, Timelike, Utc};
use std::{fs::File, io::Write, os::unix::process::CommandExt, process::Command};

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

    let date = format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day());
    let file_name = format!("{}.toml", date);
    let mut file = File::create(&file_name).expect("Error while creating file");
    let init = format!("date = \"{}\"\nmarkdown = \"\"\"\n\"\"\"", date);
    file.write_all(init.as_bytes())
        .expect("Error while writing to file");

    let mut output = Command::new("vim");
    output.arg(&file_name).status().expect("Error starting Vim");
}
