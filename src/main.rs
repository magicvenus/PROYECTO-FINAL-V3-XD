/// CLI para la gestión de tareas
///
/// Esta aplicación permite administrar una lista de tareas con las siguientes funciones:
/// - `add`: Agregar una nueva tarea a la lista.
/// - `complete`: Marcar una tarea como completada utilizando su ID.
/// - `list`: Mostrar todas las tareas existentes.
/// 
///
/// Uso:
///     cargo run -- add "Descripción de la tarea"     // Agregar una nueva tarea
///     cargo run -- complete <ID>                    // Marcar una tarea como completada
///     cargo run -- list                             // Mostrar todas las tareas
///     
///
/// Ejemplos:
///     cargo run -- add "Comprar leche"
///     cargo run -- complete 2
///     cargo run -- clear
///
/// Autor: Mario Salamanca U20210994

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add")]
    Add { task: String },
    #[structopt(name = "complete")]
    Complete { task_id: usize },
    #[structopt(name = "list")]
    List,
}

struct TaskManager {
    tasks: HashMap<usize, String>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: HashMap::new(),
        }
    }

    fn load_tasks(&mut self) -> io::Result<()> {
        if let Ok(lines) = read_lines("tasks.txt") {
            for line in lines.flatten() {
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                if let Some((index, _task)) = parts.split_first() {
                    if let Ok(index) = index.parse::<usize>() {
                        // Revisamos si hay una tarea en los parts
                        if let Some(task) = parts.get(1) {
                            self.tasks.insert(index, task.to_string());
                        }
                    }
                }
            }
            // Reindexar las tareas para asegurar que los índices sean consecutivos
            reindex_tasks(&mut self.tasks);
        }
        Ok(())
    }

    fn save_tasks(&self) -> io::Result<()> {
        let mut content = String::new();
        for (index, task) in &self.tasks {
            content.push_str(&format!("{} {}\n", index, task));
        }
        fs::write("tasks.txt", content)
    }

    fn add_task(&mut self, task: String) {
        let task_id = self.tasks.len() + 1;
        self.tasks.insert(task_id, task);
        println!("Tarea agregada con ID: {}", task_id);
    }

    fn complete_task(&mut self, task_id: usize) {
        if let Some(task) = self.tasks.remove(&task_id) {
            println!("Tarea completada: {}", task);
        } else {
            println!("No se encontró la tarea con ID: {}", task_id);
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No hay tareas pendientes.");
        } else {
            for (id, task) in &self.tasks {
                println!("ID: {}, Tarea: {}", id, task);
            }
        }
    }
}

fn reindex_tasks(tasks: &mut HashMap<usize, String>) {
    let mut new_tasks = HashMap::new();
    for (_index, task) in tasks.drain() {
        new_tasks.insert(new_tasks.len() + 1, task);
    }
    *tasks = new_tasks;
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let mut task_manager = TaskManager::new();
    task_manager.load_tasks()?; // Cargar tareas desde el archivo

    match opt.cmd {
        Command::Add { task } => {
            task_manager.add_task(task);
            task_manager.save_tasks()?; // Guardar tareas en el archivo después de agregar una nueva tarea
        }
        Command::Complete { task_id } => {
            task_manager.complete_task(task_id);
            task_manager.save_tasks()?; // Guardar tareas en el archivo después de completar una tarea
        }
        Command::List => {
            task_manager.list_tasks();
        }
    }

    Ok(())
}
