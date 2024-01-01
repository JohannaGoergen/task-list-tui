/// [[Task manager module]]
/// Manages the tasks in a to-do list as a file
/// Where each line takes the form:
/// {ID}: {status} {task description}
/// At the moment it's a single file always at $HOME/todo/list.txt

pub mod task_manager {
    use core::result::Result;
    use dirs::home_dir;
    use std::collections::HashMap;
    use std::fs::{File, OpenOptions, read_to_string};
    use std::io::prelude::*;
    use std::path::Path;
    use regex::Regex;
    use strum::IntoEnumIterator;
    use strum_macros::{EnumIter, Display};
    use itertools::Itertools;

    pub use self::TaskStatus::{Complete, Incomplete};

    #[derive(Debug, EnumIter, PartialEq, Display)]
    pub enum TaskStatus {
        Complete,
        Incomplete,
    }

    const TODO_LIST_PATH: &Path = home_dir().expect("Please set $HOME").join("todo/list.txt").as_path();
    const STATUS_OPTIONS: String = TaskStatus::iter().join("|");
    const LINE_REGEX: &str = format!(r#"\(<id>\d+)\: (<status>{}) (<description>\w*\d*\s*)"#, STATUS_OPTIONS).as_str();

    pub fn create_task(task_description: &str) -> Result<(), Box<dyn std::error::Error>>{
        let new_id_for_task = get_next_id().unwrap();
        // let new_task_line = format!(); TODO: Delete this if writeln! works.
        let mut task_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&TODO_LIST_PATH)
            .unwrap();
        writeln!(task_file, "{}: {} {}", new_id_for_task, Incomplete, task_description);
        Ok(())
    }

    pub fn edit_task(task_id: i32, status: TaskStatus, task_description: &str) -> Result<(), Box<dyn std::error::Error>> {
        //! # Edit Task
        //! Use this to edit either or both the description and status of a task
        let mut task_map = read_tasks_to_map()?;
        task_map.insert(task_id, format!("{} {}", status, task_description));
        write_tasks_to_file(task_map);
        return Ok(());
    }

    pub fn remove_task(task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut task_map = read_tasks_to_map()?;
        task_map.remove(&task_id);
        write_tasks_to_file(task_map);
        return Ok(());
    }

    pub fn read_tasks_to_map() -> Result<HashMap<i32, String>, Box<dyn std::error::Error>> {
        let mut tasks_map: HashMap<i32, String> = HashMap::new();
        let tasks_list = read_to_string(&TODO_LIST_PATH)?;
        for task_line in tasks_list.lines() {
            let line_regex = Regex::new(LINE_REGEX)?;
            let reg_match = line_regex.captures(task_line).unwrap();
            let task_id: i32 = String::from(&reg_match["id"]).parse::<i32>().unwrap();
            let task_description: &str = &reg_match["description"];
            tasks_map.insert(task_id, String::from(task_description));
        }
        return Ok(tasks_map);
    }

    // Private functions to de/serialize tasks to/from file representation
    fn write_tasks_to_file(tasks_map: HashMap<i32, String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new().write(true).open(&TODO_LIST_PATH)?;
        let mut full_tasks_list: Vec<String> = Vec::new();
        for (task_id, task_description) in tasks_map.iter() {
            full_tasks_list.push(format!("{}:{} {}", task_id, Incomplete, task_description))
        }
        file.write_all(full_tasks_list.join("\n").as_bytes());
        file.flush()?;
        return Ok(());
    }

    fn get_next_id() -> Result<i32,  Box<dyn std::error::Error>> {
        let mut file = match File::open(&TODO_LIST_PATH) {
            Err(problem) => create_file(),
            Ok(file) => file,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(problem) => panic!("Couldn't read the file."),
            Ok(_) => {
                let parts: Vec<&str>= s.lines().collect();
                if parts.len() == 0 {
                    return Ok(0);
                }
                let latest_entry = parts.last().unwrap();
                let line_regex = Regex::new(LINE_REGEX)?;
                let reg_match = line_regex.captures(latest_entry).unwrap();
                return Ok(String::from(&reg_match["id"]).parse::<i32>().unwrap() + 1);
            }
        }
    }

    fn create_file() -> File {
        let mut file = match File::create(&TODO_LIST_PATH) {
            Err(problem) => panic!("Couldn't create path"),
            Ok(file) => file,
        };
        return file;
    }
}
