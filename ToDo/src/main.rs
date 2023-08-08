use std::io;
mod Task;
mod Taskmanager;
mod dbThread;
mod taskViewModel;


use taskViewModel::ViewModel;
use uuid::Uuid;   //https://docs.rs/uuid/0.8.1/uuid/index.html#dependencies

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use rusqlite::Result;
use Task::{TaskStatus};
use dbThread::{db_thread, DbMessage};
use crate::Taskmanager::{TaskManager,SortOrder};

fn update_task_status(viewmodel:&mut ViewModel,status:TaskStatus) {
    let mut task_id_input = String::new();
    io::stdin().read_line(&mut task_id_input).expect("Failed to read line");
    let trimmed_input = task_id_input.trim();
    match Uuid::parse_str(trimmed_input) {
        Ok(task_id) => {
            // Here, 'task_id' is the parsed UUID entered by the user
            viewmodel.update_task_status(task_id,status);
        }
        Err(e) => {
            println!("Invalid taskId: {}", e);
        }
    }
}

fn main() -> Result<(), rusqlite::Error> {
    let mut viewmodel = ViewModel::new()?;
   // viewmodel.run();

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
                viewmodel.add_task(title,description,date);
            }

            2 => {
                println!("Enter task ID to mark as Done:");
                update_task_status(&mut viewmodel,TaskStatus::Done);
            }

            3 => {
                println!("Enter task ID to mark as Todo:");
                update_task_status(&mut viewmodel,TaskStatus::Todo);
            }

            4 => {
                println!("Enter task ID to mark as Archived:");
                update_task_status(&mut viewmodel,TaskStatus::Archived);
            }

            5 => {
                match viewmodel.list_tasks() {
                    Some(tasks) => {
                        for task in tasks {
                            println!(
                                "ID: {}, Title: {}, Description: {}, Date: {}, Status: {}",
                                task.id, task.title, task.description, task.date, task.status.to_string()
                            );
                        }
                    },
                    None => println!("Task List is empty: "),
                }
            }
          

            6 => {
                viewmodel.undo();
            }

            7 => {
                viewmodel.redo();
            }

            8 => {
                println!("Enter search query:");
                let mut query = String::new();
                io::stdin().read_line(&mut query).expect("Failed to read line");
                match viewmodel.search_tasks(query.trim()) {
                    Some(tasks) => {
                        for task in tasks {
                            println!(
                                "ID: {}, Title: {}, Description: {}, Date: {}, Status: {}",
                                task.id, task.title, task.description, task.date, task.status
                            );
                        }
                    }
                    None => println!("Error: in searching tasks "),
                }
            }
            9 => {
                println!("Enter 1 for Ascending  and 2 for Descending ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let sort_oder: i32 = input.trim().parse().unwrap_or(0);

                match sort_oder {
                    1 => viewmodel.sort_tasks(SortOrder::Ascending),
                    2 => viewmodel.sort_tasks(SortOrder::Descending),
                    _ => println!("Invalid choice"),
                }
            }

            1001 => {
                println!("Enter task ID to Delete:");
                
                let mut task_id_input = String::new();
                io::stdin().read_line(&mut task_id_input).expect("Failed to read line");
                let trimmed_input = task_id_input.trim();
                match Uuid::parse_str(trimmed_input) {
                    Ok(task_id) => {
                        // Here, 'task_id' is the parsed UUID entered by the user
                        viewmodel.delete_task(task_id);
                    }
                    Err(e) => {
                        println!("Invalid taskId: {}", e);
                    }
                }
            }
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
    Ok(())
}

