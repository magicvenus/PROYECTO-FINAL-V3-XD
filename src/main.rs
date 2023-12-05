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
            for (index, line) in lines.enumerate() {
                if let Ok(task) = line {
                    self.tasks.insert(index + 1, task);
                }
            }
        }
        Ok(())
    }

    fn save_tasks(&self) -> io::Result<()> {
        let mut content = String::new();
        for task in &self.tasks {
            content.push_str(&format!("{} {}\n", task.0, task.1));
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
