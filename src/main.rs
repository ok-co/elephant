use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use colored::Colorize;
use std::env;

#[derive(Serialize, Deserialize)]
struct Entry {
    name: String,
    command: String,
    note: String,
    entry_type: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let elephant_dir = home_dir.join(".elephant");
    let entries_file = elephant_dir.join("elephant.json");

    if !elephant_dir.exists() {
        fs::create_dir(&elephant_dir).expect("Could not create .elephant directory");
        println!("{}", "Created .elephant directory in home directory".bold().green());
    }

    if !entries_file.exists() {
        fs::File::create(&entries_file).expect("Could not create elephant.json file");
        println!("{}", "Created elephant.json file in .elephant directory".bold().green());
    }

    let entries = fs::read_to_string(&entries_file).expect("Could not read elephant.json file");
    let entries: Vec<Entry> = serde_json::from_str(&entries).expect("Could not parse elephant.json file");

    for entry in entries {
        match entry.entry_type.as_str() {
            "Command" => {
                println!("{}", entry.name.bold().green());
                println!("{}", entry.command.bold().blue());
            },
            "Note" => {
                println!("{}", entry.name.bold().green());
                println!("{}", entry.note.bold().blue());
            },
            "File" => {
                println!("{}", entry.name.bold().green());
                println!("{}", "File".bold().blue());
            },
            "Directory" => {
                println!("{}", entry.name.bold().green());
                println!("{}", "Directory".bold().blue());
            }
            _ => {
                println!("{}", "Invalid entry type".bold().red());
            }
        }
    }


}