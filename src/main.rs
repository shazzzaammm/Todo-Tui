use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{Seek, SeekFrom};
use std::path::Path;

const TASK_IDENTIFIER: &str = "---TASK---";
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
            "{}\n{}{}\n{}{}\n{}{}\n",
            TASK_IDENTIFIER,
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

fn get_todo_file() -> File {
    let path = Path::new("todo.txt");
    let path_display = path.display();
    let todo_file = match OpenOptions::new().append(true).read(true).open(path) {
        Ok(f) => f,
        Err(e) => panic!("Could not open {} because \'{}\'", path_display, e),
    };
    todo_file
}

fn read_todo_file(f: &mut File) -> String {
    // Move file cursor to the start
    f.seek(SeekFrom::Start(0))
        .expect("could not move cursor to the start");

    // Read the file into the buffer 'contents'
    let mut contents: String = String::new();
    match f.read_to_string(&mut contents) {
        Err(e) => panic!("Could not read todo file because \'{}\'", e),
        Ok(s) => s,
    };
    contents
}

fn add_task(task: &Task, f: &mut File) {
    // Move file cursor to the end
    f.seek(SeekFrom::End(0))
        .expect("could not move cursor to the end");

    // Write the task to the file
    match f.write_all(task.to_string().as_bytes()) {
        Ok(_) => (),
        Err(e) => println!("Could not write to file because \'{}\'", e),
    }
}

fn create_task_interactively() -> Task {
    // Get the name of the task
    println!("Task name?");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    // Get the completion status of the task
    println!("Task completed? (true or false)");
    let mut completed = String::new();
    std::io::stdin()
        .read_line(&mut completed)
        .expect("Failed to read line");

    // Parse the completion status
    let completed: bool = completed
        .trim()
        .to_lowercase()
        .parse()
        .expect("Failed to parse completion status");

    // Get the task description
    println!("Task description");
    let mut description = String::new();
    std::io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    let description = description.trim().to_string();

    // Add data to struct and return it
    Task {
        name,
        completed,
        description,
    }
}
fn main() {
    let mut todo_file = get_todo_file();
    let mut run: bool = true;

    while run {
        println!("What would you like to do?");
        println!("1: List all tasks \n2: Add a new task \n3: Quit");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();
        let input_number: i32 = match input.parse() {
            Ok(n) => n,
            _ => 0,
        };
        match input_number {
            1 => println!("{}", read_todo_file(&mut todo_file)),
            2 => add_task(&create_task_interactively(), &mut todo_file),
            3 => run = false,
            _ => println!("\'{}\' is not a valid choice!", input),
        };
    }
}
