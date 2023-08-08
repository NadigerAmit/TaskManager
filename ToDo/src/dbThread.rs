
use std::sync::mpsc;
use rusqlite::{params, Connection, Result};
use uuid::Uuid;
use crate::Task::TaskStatus;
use crate::Task::Task;

// Function to perform the actual database operations in a separate thread
pub fn db_thread(rx: mpsc::Receiver<DbMessage>, conn: Connection) {
    for message in rx {
        match message {
            DbMessage::AddTaskToDb(task) => {
                conn.execute(
                    "INSERT OR REPLACE INTO tasks (id, title,description, status, date) VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![task.id.to_string(), task.title,task.description, task.status as i32, task.date],
                ).unwrap();
            }
            DbMessage::UpdateTaskStatusInDb(task_id, status) => {
                conn.execute(
                    "UPDATE tasks SET status = ?1 WHERE id = ?2",
                    params![status as i32, task_id.to_string()],
                ).unwrap();
            }
            DbMessage::DeleteTaskFromDb(task_id) => {
                conn.execute(
                    "DELETE FROM tasks WHERE id = ?1",
                    params![task_id.to_string()],
                ).unwrap();
            }
            // Add other message types as needed
        }
    }
}

// Enum to represent different types of messages for database operations
pub enum DbMessage {
    AddTaskToDb(Task),
    UpdateTaskStatusInDb(Uuid, TaskStatus),
    DeleteTaskFromDb(Uuid),
}
