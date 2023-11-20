// Importa las macros necesarias
use std::collections::HashMap;
use structopt::StructOpt;

// Define la estructura para los argumentos de la línea de comandos
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

// Define los comandos posibles
#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add")]
    Add { task: String },
    #[structopt(name = "complete")]
    Complete { task_id: usize },
    #[structopt(name = "list")]
    List,
}

// Define la estructura principal de la aplicación
struct TaskManager {
    tasks: HashMap<usize, String>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: HashMap::new(),
        }
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
        for (id, task) in &self.tasks {
            println!("ID: {}, Tarea: {}", id, task);
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    let mut task_manager = TaskManager::new();

    match opt.cmd {
        Command::Add { task } => {
            task_manager.add_task(task);
        }
        Command::Complete { task_id } => {
            task_manager.complete_task(task_id);
        }
        Command::List => {
            task_manager.list_tasks();
        }
    }
}
