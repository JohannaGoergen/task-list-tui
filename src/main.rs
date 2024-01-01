use std::io;
use std::error::Error;

mod tasks;
use tasks::task_manager;

fn main() -> Result<(), Box<dyn Error>> {
    task_manager::create_task("Add TUI to manage tasks");
    task_manager::create_task("Add error handling");
    task_manager::edit_task(0, task_manager::TaskStatus::Complete, "Add TUI to manage tasks").expect("Failed.");
    display_tasks();
    Ok(())
}