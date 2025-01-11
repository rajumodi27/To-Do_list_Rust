use std::io;

#[derive(Debug)]
struct Todo {
    task: String,
    completed: bool,
}

impl Todo {
    fn new(task: &str) -> Self {
        Todo {
            task: task.to_string(),
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }

    fn display(&self, index: usize) {
        println!(
            "{}. [{}] {}",
            index + 1,
            if self.completed { "X" } else { " " },
            self.task
        );
    }
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    loop {
        println!("\n--- To-Do List ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match choice {
            1 => add_task(&mut todos),
            2 => view_tasks(&todos),
            3 => mark_task(&mut todos),
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_task(todos: &mut Vec<Todo>) {
    println!("Enter a new task:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();
    let task = task.trim();
    todos.push(Todo::new(task));
    println!("Task added: {}", task);
}

fn view_tasks(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("No tasks yet!");
    } else {
        println!("Here are your tasks:");
        for (i, task) in todos.iter().enumerate() {
            task.display(i);
        }
    }
}

fn mark_task(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("No tasks to mark as completed!");
        return;
    }

    println!("Enter the number of the task to mark as completed:");
    view_tasks(todos);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let index: usize = match input.trim().parse::<usize>() {
        Ok(num) => num - 1,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    
    };

    if index < todos.len() {
        todos[index].mark_completed();
        println!("Task marked as completed: {}", todos[index].task);
    } else {
        println!("Task number out of range!");
    }
}
