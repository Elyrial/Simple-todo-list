use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High
} 

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        println!("New list has been created");
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String, importance: Priority) {
        let task = Task::new(self.next_id, description, importance);
        self.tasks.push(task);
        println!("Task {} has been added", self.next_id);
        self.next_id += 1;
    }

    fn sorted_tasks(&self) -> Vec<&Task> {
        let mut sorted: Vec<&Task> = self.tasks.iter().collect();
        sorted.sort_by(|a, b| b.importance.cmp(&a.importance));
        sorted
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("The list is empty!");
            return;
        }
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

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self).expect("Failed to serialize");

        let mut file = fs::File::create(filename)?;
        file.write_all(json.as_bytes())?;

        println!("Saved file in: {}", filename);
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        let json = fs::read_to_string(filename)?;
        let todo_list: TodoList = serde_json::from_str(&json).expect("Failed to deserialize");

        Ok(todo_list)
    }
}

fn main() {

    println!("Example with loaded json file:");

    let mut loaded_todo_list = TodoList::load_from_file("example.json").expect("Failed to load from file");

    loaded_todo_list.list_tasks();

    println!();

    loaded_todo_list.add_task("Added into loaded Task".to_string(), Priority::High);
    loaded_todo_list.edit_task(1, "This COMPLETED task has NOT been edited".to_string()); // Will not be edited
    loaded_todo_list.edit_task(2, "This PENDING task has BEEN EDITED".to_string());

    println!();

    loaded_todo_list.list_tasks();

    println!("\nExample with new list instance: ");

    let mut todo_list = TodoList::new();

    todo_list.list_tasks(); // List empty list

    todo_list.add_task("First task".to_string(), Priority::Low);
    todo_list.add_task("This is another task".to_string(), Priority::Medium);
    todo_list.add_task("This is the last task of this example but with the highest priority".to_string(), Priority::High);

    todo_list.complete_task(1);

    println!();

    println!("\nListing only the completed tasks");
    todo_list.list_completed_tasks();

    println!();

    todo_list.list_tasks();
    todo_list.remove_task(2);

    println!("\nList after removing task 2");

    todo_list.list_tasks();

    let _ = todo_list.save_to_file("saved_todo_list.json");
}

