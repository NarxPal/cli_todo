use std::io; // std for standard lib, io for input/output , basically used for read/write operation
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

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new(); // vec is a growable array

    loop {
        println!("\n1. Add Task\n2. View Tasks\n3. Mark Task Done\n4. Exit");
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
                // for(index, task) in todo_list.iter().
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
                println!("Exiting game...");
                break;
            }
            _ => println!("Invalid option!"), // _ is wildcard pattern, any value other than specified will run this
        }
    }
}
