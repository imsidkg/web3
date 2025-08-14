#[derive(Debug)]
enum TaskStatus {
    Pending,
    InProgress,
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
}



fn main() {
    println!("Hello, world!");
}
