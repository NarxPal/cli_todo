// std for standard lib, io for input/output , basically used for read/write operation
use std::{fs, io};
// serialize convert rust type into format like json, binary. while deserialize convert formats into rust type
use serde::{Deserialize, Serialize};

// derive above structs and enum, help us to not manually write the impl
#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    task: String,
    done: bool,
}

// here implentation block is used to change or modify the fields in struct
impl TodoItem {
    // fn inside impl are called associated functions
    fn new_task(task: &str) -> TodoItem {
        TodoItem {
            task: task.to_string(),
            done: false,
        }
    }

    fn mark_done(&mut self) {
        // & pointing to TodoItem struct here
        // mut mean changeable since we are changing the bool here
        self.done = true;
    }
}

fn load_tasks() -> Vec<TodoItem> {
    // converting json into rust type , here struct using serde_json
    if let Ok(data) = fs::read_to_string("todo_list.json") {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<TodoItem>) {
    // convert struct into json for saving the data
    if let Ok(data) = serde_json::to_string(tasks) {
        fs::write("todo_list.json", data).expect("Unable to write file");
    }
}

fn main() {
    let mut todo_list: Vec<TodoItem> = load_tasks(); // vec is a growable array

    loop {
        println!("\n1. Add Task\n2. View Tasks\n3. Mark Task Done\n4. Delete Task\n5. Exit Game");
        let mut option = String::new(); // mut since option could change, new create empty string
        io::stdin().read_line(&mut option).unwrap(); // it read the input, unwrap for error handle
        let option: u32 = option.trim().parse().unwrap_or(0); // parse() convert string taken from input into u32(based upon type annotation)

        match option {
            1 => {
                println!("Enter task: ");
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                todo_list.push(TodoItem::new_task(task.trim()));
            }
            2 => {
                println!("All your tasks");
                let iter_list = todo_list.iter().enumerate(); // enumerate provide index value
                for (index, tasks) in iter_list {
                    let task_status = if tasks.done { "Done" } else { "Pending" };
                    println!("{}. {} , status: {} ", index + 1, tasks.task, task_status)
                    // index+ 1 will not  let 0 index for first task
                }
            }
            3 => {
                println!("Enter task number to mark as done");
                let mut task_num = String::new();
                io::stdin().read_line(&mut task_num).unwrap();
                let task_num: usize = task_num.trim().parse().unwrap_or(0);
                if task_num > 0 && task_num <= todo_list.len() {
                    todo_list[task_num - 1].mark_done(); // doing -1 for indexing, since above done index +1
                    println!("Task marked as done");
                }
            }
            4 => {
                println!("Enter task number to delete task");
                let mut del_task_num = String::new();
                io::stdin().read_line(&mut del_task_num).unwrap();
                let del_task_num: usize = del_task_num.trim().parse().unwrap_or(0);

                if del_task_num > 0 && del_task_num <= todo_list.len() {
                    todo_list.remove(del_task_num - 1); // .remove comes vec type, -1 for 0 based indexing
                    println!("Task deleted");
                    save_tasks(&todo_list);
                } else {
                    println!("Invalid Task number")
                }
            }
            5 => {
                save_tasks(&todo_list);
                println!("Exiting game...");
                break;
            }
            _ => println!("Invalid option!"), // _ is wildcard pattern, any value other than specified will run this
        }
    }
}
