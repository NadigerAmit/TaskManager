
use std::io;
mod db;
mod task;
mod viewmodel;
use viewmodel::TaskViewModel;

use crate::db::TaskManager;


fn main() {
    println!("Jai Shree Ram!");
    // Sample tasks for demonstration
    let mut view_model = TaskViewModel::new().expect("Failed to initialize TaskViewModel");


        loop {
            println!("\nTask Management Application");
            println!("1. Add Task");
            println!("2. Mark Task as Done");
            println!("3. Mark Task as Undone");
            println!("4. Archive Task");
            println!("5. List Tasks");
            println!("6. Undo");
            println!("7. Redo");
            println!("8. Search Task");
            println!("9. Sort Taks by date");
            println!("1001. delete Task");
            println!("0. Exit");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice: u32 = input.trim().parse().unwrap_or(0);
            match choice { 
                1 => {
                    println!("Enter task details:");
                    println!("Title:");
                    let mut title = String::new();
                    io::stdin().read_line(&mut title).expect("Failed to read line");

                    println!("Description:");
                    let mut description = String::new();
                    io::stdin().read_line(&mut description).expect("Failed to read line");

                    println!("Date (YYYY-MM-DD):");
                    let mut date = String::new();
                    io::stdin().read_line(&mut date).expect("Failed to read line");
                    view_model
                    .add_task(title.trim().to_string(), 
                            description.trim().to_string(), 
                            date.trim().to_string())
                        .expect("Failed to add task");
                }

                2 => {
                    println!("Enter task ID to mark as Done:");
                    let mut task_id_input = String::new();
                    io::stdin().read_line(&mut task_id_input).expect("Failed to read line");
                    let task_id: i32 = task_id_input.trim().parse().unwrap_or(0);
                    match view_model.mark_task_done(task_id) {
                        Ok(_) => println!("Task marked as Done."),
                        Err(err) => println!("{}", err),
                    }
                }

                3 => {
                    println!("Enter task ID to mark as Undone:");
                    let mut task_id_input = String::new();
                    io::stdin().read_line(&mut task_id_input).expect("Failed to read line");
                    let task_id: i32 = task_id_input.trim().parse().unwrap_or(0);
                    match view_model.mark_task_done(task_id) {
                        Ok(_) => println!("Task marked as Undone."),
                        Err(err) => println!("{}", err),
                    }
                }

                4 => {
                    println!("Enter task ID to archive:");
                    let mut task_id_input = String::new();
                    io::stdin().read_line(&mut task_id_input).expect("Failed to read line");
                    let task_id: i32 = task_id_input.trim().parse().unwrap_or(0);
    
                    match view_model.archive_task(task_id) {
                        Ok(_) => println!("Task archived."),
                        Err(err) => println!("{}", err),
                    }
                }

                5 => {
                    match view_model.list_tasks() {
                        Ok(tasks) => {
                            for task in tasks {
                                println!(
                                    "ID: {}, Title: {}, Description: {}, Date: {}, Status: {}",
                                    task.id, task.title, task.description, task.date, task.status
                                );
                            }
                        }
                        Err(err) => println!("Error: {}", err),
                    }
                }

                6 => {
                    match view_model.undo() {
                        Ok(_) => println!("Undo successful."),
                        Err(err) => println!("{}", err),
                    }
                }

                7 => {
                    match view_model.redo() {
                        Ok(_) => println!("Redo successful."),
                        Err(err) => println!("{}", err),
                    }
                }

                8 => {
                    println!("Enter search query:");
                    let mut query = String::new();
                    io::stdin().read_line(&mut query).expect("Failed to read line");
                    match view_model.search_tasks(query.trim()) {
                        Ok(tasks) => {
                            for task in tasks {
                                println!(
                                    "ID: {}, Title: {}, Description: {}, Date: {}, Status: {}",
                                    task.id, task.title, task.description, task.date, task.status
                                );
                            }
                        }
                        Err(err) => println!("Error: {}", err),
                    }

                    match view_model.redo() {
                        Ok(_) => println!("Redo successful."),
                        Err(err) => println!("{}", err),
                    }
                }
                9 => {
                    println!("Sorting the error");
                    match view_model.sort_by_due_date() {
                        Ok(_) => {},
                        Err(err) => {
                            println!("{}", err)
                        }
                    }
                }

                1001 => {
                    println!("Enter task ID to Delete:");
                    let mut task_id_input = String::new();
                    io::stdin().read_line(&mut task_id_input).expect("Failed to read line");
                    let task_id: i32 = task_id_input.trim().parse().unwrap_or(0);
                    match view_model.delete_task(task_id) {
                        Ok(_) => println!("Redo successful."),
                        Err(err) => println!("{}", err),
                    }
                }

                0 => break,
                _ => println!("Invalid choice."),
            }
        }
}
