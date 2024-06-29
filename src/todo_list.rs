// src/todo_list.rs
use crate::task::Task;
use crate::utils::{load_tasks, save_tasks};

pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            tasks: Vec::new(),
        }
    }

    pub fn load_from_file() -> std::io::Result<TodoList> {
        let tasks = load_tasks()?;
        Ok(TodoList { tasks })
    }

    pub fn add_task(&mut self, description: String) {
        let id = self.tasks.len() + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!("{:?}", task);
        }
    }

    pub fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
        }
    }

    pub fn remove_task(&mut self, id: usize) {
        self.tasks.retain(|task| task.id != id);
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        save_tasks(&self.tasks)
    }
}
