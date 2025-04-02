#[derive(Debug, PartialEq)]
enum TaskStatus {
    Pending,
    Completed,
}

struct Task {
    id: usize,
    description: String,
    status: TaskStatus,
}

impl Task {
    fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            status: TaskStatus::Pending,
        }
    }

    fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        self.next_id += 1;
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "ID: {} | {} | Status: {:?}",
                task.id, task.description, task.status
            );
        }
    }

    fn complete_task(&mut self, task_id: usize)  {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.complete();
            println!("Task {} has been marked as complete", task_id);
        } else {
            println!("Task could not be found");
        }
    }

    fn edit_task(&mut self, task_id: usize, description: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            if task.status == TaskStatus::Completed {
                println!("Task {} is already completed, it can not be edited", task_id)
            } else {
                task.description = description;
                println!("Task {} has been edited successfully!", task_id)
            }
        } else {
            println!("Task could not be found");
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.add_task("Learn Rust".to_string());
    todo_list.add_task("Build a todo list".to_string());

    todo_list.list_tasks();

    todo_list.complete_task(1);
    todo_list.edit_task(2, "This task was updated".to_string());
    todo_list.list_tasks();

    todo_list.edit_task(1, "Trying to update completed task".to_string());

}




