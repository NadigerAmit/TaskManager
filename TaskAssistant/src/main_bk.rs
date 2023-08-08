mod Task;
mod Taskmanager;
mod dbThread;
mod viewmodel;

use viewmodel::ViewModel;

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use rusqlite::Result;
use Task::{TaskStatus};
use dbThread::{db_thread, DbMessage};
use crate::Taskmanager::TaskManager;


fn main() -> Result<()> {
    // Initialize the task manager
    let task_manager = Arc::new(Mutex::new(TaskManager::new()?));

    // Create a channel for communication between the main thread and the db thread
    let (tx, rx) = mpsc::channel();

    // Clone the task manager for the database thread
    let db_task_manager = task_manager.clone();

    // Spawn the db_thread
    thread::spawn(move || {
        let conn = rusqlite::Connection::open("tasks.db").expect("Error opening the database");
        let task_manager = Arc::new(Mutex::new(TaskManager::new()));
    });

    // Example usage
    let task_id1 = task_manager.lock().unwrap().add_task("Task 1".to_string(), "2023-07-27".to_string(), &tx);
    let task_id2 = task_manager.lock().unwrap().add_task("Task 2".to_string(), "2023-07-28".to_string(), &tx);

    // Mark a task as "Done" (simulated async operation)
    let tx1 = tx.clone();
    thread::spawn(move || {
        // Simulate the async operation
        thread::sleep(std::time::Duration::from_millis(50));
        tx1.send(DbMessage::UpdateTaskStatusInDb(task_id1, TaskStatus::Done)).unwrap();
    });

    // Mark another task as "Done" (simulated async operation)
    let tx2 = tx.clone();
    thread::spawn(move || {
        // Simulate the async operation
        thread::sleep(std::time::Duration::from_millis(50));
        tx2.send(DbMessage::UpdateTaskStatusInDb(task_id2, TaskStatus::Done)).unwrap();
    });

    // Wait for a short time to allow async operations to complete
    thread::sleep(std::time::Duration::from_millis(100));

    // List all tasks
    let tasks = task_manager.lock().unwrap().list_tasks();
    println!("All tasks: {:?}", tasks);

    // Search for tasks containing "Task 1"
    let tasks_with_keyword = task_manager.lock().unwrap().search_tasks("Task 1");
    println!("Tasks with keyword 'Task 1': {:?}", tasks_with_keyword);

    // Close the database connection
    Ok(())
}
