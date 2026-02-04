use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::File,
    io::{BufReader, Write},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Add(String),
    Delete(usize),
    Finish(usize),
    List,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks = match File::open("tasks.json") {
        Ok(file) => {
            println!("Loading tasks from file.");
            load_tasks(&file)
        }
        Err(e) => {
            eprintln!("{}", e);
            println!("No task file found, creating new one.");
            File::create_new("tasks.json").expect("Should create file");
            Vec::new()
        }
    };

    let command: Command = match parse_args(&args) {
        Some(cmd) => cmd,
        None => {
            usage_help();
            return;
        }
    };

    match command {
        Command::Add(title) => {
            add_task(&mut tasks, title);
            match save_to_file(&tasks) {
                Ok(_) => println!("Saved to file."),
                Err(e) => eprintln!("{}", e),
            }
        }
        Command::Delete(id) => {
            delete_task(&mut tasks, id);
            match save_to_file(&tasks) {
                Ok(_) => println!("Saved to file."),
                Err(e) => eprintln!("{}", e),
            }
        }
        Command::Finish(id) => {
            finish_task(&mut tasks, id);
            match save_to_file(&tasks) {
                Ok(_) => println!("Saved to file."),
                Err(e) => eprintln!("{}", e),
            }
        }
        Command::List => list_tasks(&tasks),
    };
}

fn usage_help() {
    println!(
        "Command not found\nUsage:\ntodo add <Task>\ntodo delete <Number>\ntodo finish <Number>\ntodo list"
    );
}

pub fn parse_args(args: &[String]) -> Option<Command> {
    match args.get(1)?.as_str() {
        "add" => Some(Command::Add(args.get(2..)?.join(" "))),
        "delete" => Some(Command::Delete(args.get(2)?.parse::<usize>().ok()?)),
        "finish" => Some(Command::Finish(args.get(2)?.parse::<usize>().ok()?)),
        "list" => Some(Command::List),
        _ => None,
    }
}

pub fn add_task(tasks: &mut Vec<Task>, title: String) {
    let id = tasks.len() + 1;
    tasks.push(Task {
        id,
        title,
        completed: false,
    });
    println!("Added task with number {id}.");
}

pub fn delete_task(tasks: &mut Vec<Task>, id: usize) {
    match tasks.iter().position(|t| t.id == id) {
        Some(pos) => {
            tasks.remove(pos);
            println!("Deleted task with id {id}.");
        }
        None => {
            println!("Task with id {id} not found.");
            return;
        }
    };

    update_ids(tasks)
}

fn update_ids(tasks: &mut [Task]) {
    for (index, task) in tasks.iter_mut().enumerate() {
        task.id = index + 1;
    }
}

pub fn finish_task(tasks: &mut [Task], id: usize) {
    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            task.completed = !task.completed;
            println!("Task with id {id} changed.");
        }
        None => println!("Task with id {id} not found."),
    }
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    for task in tasks {
        println!(
            "{}. [{}] {}",
            task.id,
            if task.completed { "x" } else { " " },
            task.title
        )
    }
}

pub fn save_to_file(tasks: &[Task]) -> Result<()> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .open("tasks.json")?;
    let tasks = serde_json::to_string(tasks)?;
    file.write_all(tasks.as_bytes())?;
    Ok(())
}

pub fn load_tasks(file: &File) -> Vec<Task> {
    let reader = BufReader::new(file);
    let tasks: Vec<Task> = match serde_json::from_reader(reader) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            println!("Data was not well-formed!");
            Vec::new()
        }
    };
    tasks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list_command_correctly() {
        let args = vec!["target\\debug\\todo.exe".to_owned(), "list".to_owned()];
        let result = parse_args(&args);
        assert_eq!(result, Some(Command::List));
    }

    #[test]
    fn parse_add_command_correctly() {
        let args = vec![
            "target\\debug\\todo.exe".to_owned(),
            "add".to_owned(),
            "Task".to_owned(),
        ];
        let result = parse_args(&args);
        assert_eq!(result, Some(Command::Add("Task".to_owned())));
    }

    #[test]
    fn parse_delete_command_correctly() {
        let args = vec![
            "target\\debug\\todo.exe".to_owned(),
            "delete".to_owned(),
            "1".to_owned(),
        ];
        let result = parse_args(&args);
        assert_eq!(result, Some(Command::Delete(1)));
    }

    #[test]
    fn parse_finish_command_correctly() {
        let args = vec![
            "target\\debug\\todo.exe".to_owned(),
            "finish".to_owned(),
            "2".to_owned(),
        ];
        let result = parse_args(&args);
        assert_eq!(result, Some(Command::Finish(2)));
    }
}
