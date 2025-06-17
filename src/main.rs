use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const NAME_IDENTIFIER: &str = "NAME: ";
const COMPLETED_IDENTIFIER: &str = "COMPLETED: ";
const DESCRIPTION_IDENTIFIER: &str = "DESCRIPTION: ";

struct Task {
    name: String,
    completed: bool,
    description: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}\n{}{}\n{}{}",
            NAME_IDENTIFIER,
            self.name,
            COMPLETED_IDENTIFIER,
            self.completed,
            DESCRIPTION_IDENTIFIER,
            self.description
        )
    }
}

/// Parse a task from its string
fn parse_task_string(s: &str) -> Task {
    // Create a new empty task
    let mut task = Task {
        name: String::new(),
        completed: false,
        description: String::new(),
    };

    // Read the task string into lines
    let mut lines = s.lines();

    // Get the line with the name in it and put the name as not found otherwise
    let name_line = match lines.next() {
        Some(s) => s.to_string(),
        _ => format!("NO NAME FOUND"),
    };

    // Parse the name
    task.name = name_line.replace(NAME_IDENTIFIER, "");

    // Get the line with completion status or set it to false
    let completed_line = match lines.next() {
        Some(s) => s.to_string(),
        _ => String::from("false"),
    };

    // Parse the completion status
    task.completed = match completed_line.replace(COMPLETED_IDENTIFIER, "").parse() {
        Ok(b) => b,
        _ => false,
    };

    // Read remaining lines as the description
    for line in lines {
        task.description += line;
        task.description += "\n";
    }

    // Parse the description
    task.description = task
        .description
        .replace(DESCRIPTION_IDENTIFIER, "")
        .trim()
        .to_string();

    // Return our parsed task
    task
}

fn main() {
    let path = Path::new("todo.txt");
    let path_display = path.display();
    let mut f = match File::open(path) {
        Err(e) => panic!("Could not open {} because \'{}\'", path_display, e),
        Ok(file) => file,
    };

    let mut contents: String = String::new();
    match f.read_to_string(&mut contents) {
        Err(e) => panic!("Could not read {} because \'{}\'", path_display, e),
        Ok(_) => (),
    };

    let mut split_contents = contents.split("---TASK---");
    split_contents.next();

    for task_string in split_contents {
        let task = parse_task_string(task_string.trim());
        println!("{}", task.to_string());
    }
}
