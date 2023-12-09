# PROYECTO-FINAL-V4-XD

*****en la carpeta (target/debug/fingerprints) cambiar el nombre de la carpeta de fingerprint a .fingerprint*****
Gestor de Tareas CLI en Rust
Este programa simple de línea de comandos (CLI) desarrollado en Rust te permite administrar una lista de tareas de forma eficiente. Puedes agregar nuevas tareas, marcar tareas como completadas y  listar las tareas existentes.

*Características:*
Interfaz de línea de comandos fácil de usar.

*Funcionalidades principales:*
Agregar tareas.
Marcar tareas como completadas.
Mostrar la lista de tareas.

*Uso*
Requerimientos
Rust instalado

*Instalación*
Clona este repositorio en tu máquina local.
Accede al directorio del proyecto.

Ejecución
Para compilar y ejecutar el programa, utiliza el siguiente comando desde la terminal:

bash
Copy code
cargo run -- <comando> [opciones]
Reemplaza <comando> con uno de los siguientes:

add: Agrega una nueva tarea. Ejemplo: cargo run -- add --task "Descripción de la tarea"
complete: Marca una tarea como completada. Ejemplo: cargo run -- complete --task_id 2
list: Muestra la lista de tareas. Ejemplo: cargo run -- list

Reemplaza [opciones] con los parámetros correspondientes a cada comando.

Estructura del Proyecto
src/: Contiene el código fuente de la aplicación.
tasks.txt: Archivo para almacenar las tareas.

Contribuciones
Uso del STRUCT OTP

Autor
Mario Salamanca U20210994
