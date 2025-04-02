#[derive(Debug, PartialEq)]
enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq)]
enum Priority {
    Low,
    Medium,
    High
} 

struct Task {
    id: usize,
    description: String,
    importance: Priority,
    status: TaskStatus,
}

impl Task {
    fn new(id: usize, description: String, importance: Priority) -> Self {
        Self {
            id,
            description,
            importance,
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

    fn add_task(&mut self, description: String, importance: Priority) {
        let task = Task::new(self.next_id, description, importance);
        self.tasks.push(task);
        self.next_id += 1;
    }

    fn sorted_tasks(&self) -> Vec<&Task> {
        let mut sorted: Vec<&Task> = self.tasks.iter().collect();
        sorted.sort_by(|a, b| b.importance.cmp(&a.importance));
        sorted
    }

    fn list_tasks(&self) {
        for task in &self.sorted_tasks() {
            println!(
                "ID: {} | {} | Status: {:?} | Priority: {:?}",
                task.id, task.description, task.status, task.importance
            );
        }
    }

    // Instead of seperate functions, create a generic filtering system
    fn list_completed_tasks(&self) {
        for task in &self.tasks {
            if task.status == TaskStatus::Completed {
                println!(
                    "ID: {} | {} | Status: {:?} | Priority: {:?}",
                    task.id, task.description, task.status, task.importance
                );
            }
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

    fn remove_task(&mut self, task_id: usize) {
        if let Some(pos) = self.tasks.iter_mut().position(|t| t.id == task_id) {
            self.tasks.remove(pos);
            println!("Task {} has been successfully removed", task_id);
        } else {
            println!("Task could not be found");
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.add_task("Learn Rust".to_string(), Priority::Medium);
    todo_list.add_task("Build a todo list".to_string(), Priority::High);

    todo_list.list_tasks();

    todo_list.complete_task(1);
    todo_list.edit_task(2, "This task was updated".to_string());
    todo_list.list_tasks();

    todo_list.edit_task(1, "Trying to update completed task".to_string());

    todo_list.add_task("3rd element todo".to_string(), Priority::Low);
    todo_list.list_tasks();
    todo_list.remove_task(2);
    todo_list.list_tasks();
    todo_list.add_task("High priority task".to_string(), Priority::High);
    todo_list.list_tasks();

    todo_list.complete_task(3);
    todo_list.list_completed_tasks();

}




