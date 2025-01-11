use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Todo {
    task: String,
    completed: bool,
}

impl Todo {
    fn new(task: &str) -> Todo {
        Todo {
            task: task.to_string(),
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }

    fn display(&self) {
        println!("[{}] {}", if self.completed { "X" } else { " " }, self.task);
    }
}

fn main() {
    println!("Welcome to the To-Do List app!");

    // Simple logic to interact with the user
    loop {
        println!("\nMenu:");
        println!("1. Add a task");
        println!("2. List tasks");
        println!("3. Mark task as completed");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap();

        match choice {
            1 => add_task(),
            2 => list_tasks(),
            3 => mark_completed(),
            4 => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_task() {
    println!("Enter the task:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();
    let task = task.trim();

    let todo = Todo::new(task);
    save_task(todo);

    println!("Task added successfully!");
}

fn save_task(todo: Todo) {
    let path = Path::new("tasks.txt");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .unwrap();

    writeln!(file, "{}|{}", todo.task, todo.completed).unwrap();
}

fn list_tasks() {
    let path = Path::new("tasks.txt");
    if !path.exists() {
        println!("No tasks to show.");
        return;
    }

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('|').collect();
        let task = parts[0];
        let completed = parts[1] == "true";

        let todo = Todo {
            task: task.to_string(),
            completed,
        };

        todo.display();
    }
}

fn mark_completed() {
    let path = Path::new("tasks.txt");
    if !path.exists() {
        println!("No tasks to mark as completed.");
        return;
    }

    let mut tasks: Vec<Todo> = Vec::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('|').collect();
        let task = parts[0];
        let completed = parts[1] == "true";

        tasks.push(Todo {
            task: task.to_string(),
            completed,
        });
    }

    println!("Enter the task number to mark as completed:");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {}", i + 1, task.task);
    }

    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).unwrap();
    let task_number: usize = task_number.trim().parse().unwrap();

    if task_number > 0 && task_number <= tasks.len() {
        let mut task = &mut tasks[task_number - 1];
        task.mark_completed();
        println!("Task marked as completed.");
        save_all_tasks(&tasks);
    } else {
        println!("Invalid task number.");
    }
}

fn save_all_tasks(tasks: &Vec<Todo>) {
    let path = Path::new("tasks.txt");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();

    for task in tasks {
        writeln!(file, "{}|{}", task.task, task.completed).unwrap();
    }
}
