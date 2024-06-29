// src/utils.rs
use crate::task::Task;
use std::fs;

const FILE_PATH: &str = "tasks.json";

pub fn save_tasks(tasks: &Vec<Task>) -> std::io::Result<()> {
    let serialized = serde_json::to_string(tasks)?;
    fs::write(FILE_PATH, serialized)?;
    Ok(())
}

pub fn load_tasks() -> std::io::Result<Vec<Task>> {
    let data = fs::read_to_string(FILE_PATH).unwrap_or("[]".to_string());
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}
