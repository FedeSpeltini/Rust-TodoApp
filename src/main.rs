// src/main.rs
mod task;
mod todo_list;
mod utils;

use todo_list::TodoList;
use std::io::{self, Write};

fn main() {
    let mut todo_list = TodoList::load_from_file().unwrap_or_else(|_| TodoList::new());

    loop {
        println!("\n=== Gestor de Tareas ===");
        println!("1. Agregar tarea");
        println!("2. Listar tareas");
        println!("3. Completar tarea");
        println!("4. Eliminar tarea");
        println!("5. Salir");
        print!("Elige una opción: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                print!("Descripción de la tarea: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                let description = description.trim().to_string();
                todo_list.add_task(description);
            }
            2 => todo_list.list_tasks(),
            3 => {
                print!("ID de la tarea a completar: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: usize = id.trim().parse().unwrap_or(0);
                todo_list.complete_task(id);
            }
            4 => {
                print!("ID de la tarea a eliminar: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id: usize = id.trim().parse().unwrap_or(0);
                todo_list.remove_task(id);
            }
            5 => {
                todo_list.save_to_file().unwrap();
                break;
            }
            _ => println!("Opción no válida, intenta de nuevo."),
        }

        todo_list.save_to_file().unwrap();
    }
}
