use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use termion::style;

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    task: String,
    done: bool,
}

fn add_todo(todo_list: &mut Vec<TodoItem>, task: String) {
    let new_item = TodoItem { task, done: false };
    todo_list.push(new_item);
}

fn remove_todo(todo_list: &mut Vec<TodoItem>, task_index: usize) {
    todo_list.remove(task_index);
}

fn update(todo_list: &mut Vec<TodoItem>, task_index: usize) {
    let task = &mut todo_list[task_index];
    task.done = !task.done;
}

fn display_list(todo_list: &Vec<TodoItem>) {
    for (index, item) in todo_list.iter().enumerate() {
        if item.done {
            println!("{}: [x] {}", index, item.task);
        } else {
            println!("{}: [ ] {}", index, item.task);
        }
    }
}

fn save_list(todo_list: &Vec<TodoItem>) {
    let serialized = serde_json::to_string(todo_list).unwrap();
    let mut file = File::create("todo_list.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}

fn load_list() -> Vec<TodoItem> {
    let mut file = File::open("todo_list.json").unwrap_or_else(|_| {
        File::create("todo_list.json").unwrap();
        File::open("todo_list.json").unwrap()
    });

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    match serde_json::from_str(&contents) {
        Ok(todo_list) => todo_list,
        Err(_) => Vec::new(),
    }
}

fn show_prompt() -> String {
    let mut input = String::new();
    print!("{}[todo]: {}", style::Bold, style::Reset);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<String>().unwrap()
}

fn parse_usize() -> usize {
    let index: usize = match show_prompt().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Could not parse string as usize");
            return std::usize::MAX;
        }
    };
    return index;
}

fn main() {
    let mut todo_list = load_list();
    println!();
    println!("{}Todo List Manager{}", style::Bold, style::Reset);
    println!("type help for help");
    loop {
        let action = show_prompt();
        let action = action.as_str();

        match action {
            "help" => {
                println!("Commands: add, remove, update, size, show, quit")
            }
            "add" => {
                println!("Enter the task description:");
                let task = show_prompt();
                add_todo(&mut todo_list, task);
            }
            "size" => {
                println!("The length of your list is: {}", todo_list.len());
            }
            "remove" => {
                display_list(&todo_list);
                println!("Enter the task index:");
                let index = parse_usize();
                remove_todo(&mut todo_list, index);
            }
            "update" => {
                display_list(&todo_list);
                println!("Enter the task index:");
                let index = parse_usize();
                update(&mut todo_list, index);
            }
            "show" => display_list(&todo_list),
            "quit" => {
                println!("List saved.");
                save_list(&todo_list);
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}
