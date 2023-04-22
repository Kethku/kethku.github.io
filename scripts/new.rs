//! ```cargo
//! [dependencies]
//! chrono = "0.4.24"
//! convert_case = "0.6.0"
//! ```

use std::io::Write;

use convert_case::{Case, Casing};

fn main() {
    // Go to ./content/trio/maple/ and find all the files and directories that
    // start with "day##-"
    let mut days: Vec<String> = Vec::new();
    for entry in std::fs::read_dir("./content/trio/maple/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let file_name_string = file_name.to_string_lossy().to_owned();
        if file_name_string.starts_with("day") {
            days.push(file_name_string.to_string());
        }
    }

    // Parse the ## from each day and find the max
    let mut max = 0;
    for day in days {
        let day = day.split("-").next().unwrap();
        let day = day.split("day").last().unwrap();
        let day = day.parse::<i32>().unwrap();
        if day > max {
            max = day;
        }
    }

    // Create the new day name using the passed in commandline argument
    // as the suffix
    let day_number = max + 1;
    let title = std::env::args().nth(1).unwrap().to_case(Case::Kebab);
    let new_day = format!("day{day_number}-{title}");

    // Create a new directory for the new day and create an index.md file
    // in it with a header formatted like so:
    //
    //   +++
    //   title = "Day<number> - <suffix>"
    //   description = ""
    //   date = <date>
    //   +++
    //
    //  With the date being the current date in the format of YYYY-MM-DD
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let new_day_path = format!("./content/trio/maple/{new_day}");
    std::fs::create_dir(&new_day_path).unwrap();
    let index_path = format!("{new_day_path}/index.md");
    let title = title.to_case(Case::Title);
    std::fs::write(
        index_path.clone(),
        format!(
            r#"+++
title = "Day{day_number} - {title}"
description = ""
date = {date}
+++

"#,
        ),
    );

    // Print out the index.md file path so that the user can open it in
    // their editor of choice
    println!("{}", index_path);
}
