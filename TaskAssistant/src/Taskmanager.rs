use std::collections::{HashMap, VecDeque};
use rusqlite::{params, Connection, Result, types::FromSql};
use crate::Task::{Task, TaskStatus};

use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};
use crate::dbThread::DbMessage;
use uuid::Uuid;
use chrono::NaiveDate;

pub struct TaskManager {
    pub tasks: HashMap<Uuid, Task>,
    pub db_conn: Arc<Mutex<Connection>>,
    task_history: VecDeque<Task>, // Store snapshots for undo
    undone_tasks: VecDeque<Task>, // Store snapshots for redo
}

pub enum SortOrder {
    Ascending,
    Descending,
}


#[derive(Debug)]
struct InvalidStatusError;

impl std::fmt::Display for InvalidStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid status value found in the database")
    }
}

impl std::error::Error for InvalidStatusError {}

impl TaskManager {
    pub fn new() -> Result<Self> {
        let db_conn = Connection::open("tasks.db")?;
        //let db_conn_clone = db_conn.clone();
        db_conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT NOT NULL PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                status INTEGER NOT NULL,
                date TEXT NOT NULL
            )",
            params![],
        )?;

        let mut task_manager = TaskManager {
            tasks: HashMap::new(),
            task_history:VecDeque::new(),
            undone_tasks:VecDeque::new(),
            db_conn: Arc::new(Mutex::new(db_conn)), // Wrap the Connection in an Arc<Mutex>
        };
        
        task_manager.load_tasks_from_db()?;
        Ok(task_manager)
    }

    pub fn add_task(&mut self, title: String,description: String, date: String, tx: &mpsc::Sender<DbMessage>) -> Uuid {
        let task = Task::new(title,description, date);
        self.tasks.insert(task.id, task.clone()); // this is adding in the cache
        tx.send(DbMessage::AddTaskToDb(self.tasks[&task.id].clone())).unwrap(); // Save the new task to the database
        task.id
    }

    pub fn update_task_status(&mut self, task_id: Uuid, status: TaskStatus, tx: &mpsc::Sender<DbMessage>) -> bool {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            // Add the current task snapshot to the task history
            self.task_history.push_front(task.clone());
            task.status = status.clone();
            tx.send(DbMessage::UpdateTaskStatusInDb(task_id, status.clone())).unwrap(); // Save the updated task to the database
            // Clear the undone tasks history, as a new change has been made
            self.undone_tasks.clear();
            true
        } else {
            false
        }
    }

    pub fn undo(&mut self, tx: &mpsc::Sender<DbMessage>) -> bool {

        if let Some(task_with_updated_status) = self.task_history.pop_front() {
            // Move the current task with updated status to the undone tasks history for redo
            self.undone_tasks.push_front(self.tasks.get(&task_with_updated_status.id).unwrap().clone());
            // Restore the previous status as the current status
            let task = self.tasks.get_mut(&task_with_updated_status.id).unwrap();
            task.status = task_with_updated_status.status;
            tx.send(DbMessage::UpdateTaskStatusInDb(task.id,  task.status.clone())).unwrap(); // Save the updated task to the database
            true
        } else {
            println!("TaskManagerError::UndoHistoryEmpty");
            false
        }
    }

    pub fn redo(&mut self, tx: &mpsc::Sender<DbMessage>) -> bool {

        if let Some(task_with_undone_status) = self.undone_tasks.pop_front() {
            // Move the current task with undone status to the history for undo
            self.task_history.push_front(self.tasks.get(&task_with_undone_status.id).unwrap().clone());
            // Restore the undone status as the current status
            let task = self.tasks.get_mut(&task_with_undone_status.id).unwrap();
            task.status = task_with_undone_status.status;

            // Send a message to the db_thread to update the task status in the database
            tx.send(DbMessage::UpdateTaskStatusInDb(task.id, task.status.clone())).unwrap();
            true
        } else {
            println!("Err(TaskManagerError::RedoHistoryEmpty)");
            false
        }
    }

    pub fn delete_task(&mut self, task_id: Uuid, tx: &mpsc::Sender<DbMessage>) -> bool {
        if self.tasks.remove(&task_id).is_some() {
            tx.send(DbMessage::DeleteTaskFromDb(task_id)).unwrap(); // Delete the task from the database
            true
        } else {
            false
        }
    }

    pub fn list_tasks(&self) -> Vec<Task> {
        self.tasks.values().cloned().collect()
    }

    pub fn search_tasks(&self, keyword: &str) -> Vec<Task> {
        self.tasks
            .values()
            .filter(|task| task.title.contains(keyword) ||task.description.contains(keyword) || task.date.contains(keyword))
            .cloned()
            .collect()
    }

    pub fn sort_tasks_by_date(&mut self, order: SortOrder) {
        println!("sort_tasks_by_date eneterd");
        let mut sorted_tasks: Vec<Task> = self.tasks.values().cloned().collect();
        
        sorted_tasks.sort_by(|a, b| {
            let date_a = &a.date;
            let date_b = &b.date;
            match order {
                SortOrder::Ascending => date_a.cmp(date_b),
                SortOrder::Descending => date_b.cmp(date_a),
            }
        });
        self.tasks.clear();
        for task in sorted_tasks {
            println!("{:?}",task);
        }
    }

    pub fn save_task_to_db(&self, conn: &Connection, task: &Task) -> Result<()> {
        let conn = self.db_conn.lock().unwrap(); // Lock the Mutex to access the underlying Connection

        conn.execute(
            "INSERT OR REPLACE INTO tasks (id, description, status, date) VALUES (?1, ?2, ?3, ?4)",
            params![task.id.to_string(), task.description, task.status.clone() as i32, task.date],
        )?;
        Ok(())
    }

    pub fn load_tasks_from_db(&mut self) -> Result<()> {
        let conn = self.db_conn.lock().unwrap(); // Lock the Mutex to access the underlying Connection
        let mut stmt = conn.prepare("SELECT * FROM tasks")?;
        let task_iter = stmt.query_map(params![], |row| {
            let id_str = row.get_ref(0)?.as_str().or_else(|_| Err(rusqlite::Error::ToSqlConversionFailure(Box::new(InvalidStatusError))))?;
            let id = Uuid::parse_str(id_str).map_err(|_| rusqlite::Error::ToSqlConversionFailure(Box::new(InvalidStatusError)))?;
            //let id = row.get_ref(0)?.as_str()?.and_then(|value| Uuid::parse_str(value).ok());
            //let id = id.ok_or_else(|| rusqlite::Error::InvalidColumnType(0, "Invalid UUID".into()))?;
            //let id = id.ok_or_else(|| rusqlite::Error::ToSqlConversionFailure(Box::new(InvalidStatusError)))?;
            
            let status_value: i32 = row.get(3)?;
            let status = match status_value {
                0 => TaskStatus::Todo,
                1 => TaskStatus::Done,
                2 => TaskStatus::Archived,
                _ => return Err(rusqlite::Error::ToSqlConversionFailure(Box::new(InvalidStatusError))),
                //_ => return Err(Box::new(InvalidStatusError)),
            };

            Ok(Task {
                id,
                title: row.get(1)?,
                description: row.get(2)?,
                status,
                date: row.get(4)?,
            })
        })?;

        self.tasks.clear();
        for task in task_iter {
            let task = task?;
            self.tasks.insert(task.id, task);
        }

        Ok(())
    }
}