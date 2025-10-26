#[derive(Debug)]
enum TaskStatus {
    Pending,
    // InProgress,
    Completed,
}

struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
    due_date: Option<String>,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        return Self { tasks: Vec::new() };
    }

    fn add_task(&mut self, description: String, due_date: Option<String>) {
        let id = self.tasks.len() as u32 + 1;
        self.tasks.push(Task {
            id,
            description,
            status: TaskStatus::Pending,
            due_date,
        });
    }

    fn list_tasks(&self) {
        println!(
            "
--- To-Do List ---"
        );
        for task in &self.tasks {
            print!(
                "ID: {} - {} (Status: {:?})",
                task.id, task.description, task.status
            );
            match &task.due_date {
                Some(date) => {
                    print!(" - Due: {}", date);
                }
                None => {}
            }
            println!();
        }
        println!("--------------------");
    }

    fn mark_completed(&mut self, id: u32) -> Result<(), String> {
        for task in &mut self.tasks {
            if task.id == id {
                task.status = TaskStatus::Completed;
                return Ok(());
            }
        }
        return Err("Task not found".to_string());
    }
}

fn main() {
    use std::io::{self, Write};

    let mut todo_list = TodoList::new();

    loop {
        println!("\nMenu:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Exit");
        print!("Please enter your choice: ");

        io::stdout().flush().expect("Failed to flush stdout");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                print!("Enter the task description: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");

                print!("Enter a due date (optional, press Enter to skip): ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut due_date_input = String::new();
                io::stdin()
                    .read_line(&mut due_date_input)
                    .expect("Failed to read line");

                let description = description.trim().to_string();
                let due_date = if due_date_input.trim().is_empty() {
                    None
                } else {
                    Some(due_date_input.trim().to_string())
                };

                if !description.is_empty() {
                    todo_list.add_task(description, due_date);
                    println!("Task added successfully!");
                } else {
                    println!("Task description cannot be empty.");
                }
            }
            "2" => todo_list.list_tasks(),
            "3" => {
                if todo_list.tasks.is_empty() {
                    println!("\nThere are no tasks to mark as completed.");
                }

                println!();
                todo_list.list_tasks();
                print!("Enter the ID of the task to complete: ");
                io::stdout().flush().expect("Failed to flush stdout");

                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");

                match id_input.trim().parse::<u32>() {
                    Ok(id) => match todo_list.mark_completed(id) {
                        Ok(()) => println!("Task marked as completed!"),
                        Err(e) => println!("Error: {}", e),
                    },
                    Err(_) => {
                        println!("Invalid ID. Please enter a valid number.");
                    }
                }
            }
            "4" => {
                println!("Exiting.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
