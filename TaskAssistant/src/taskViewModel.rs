use crate::Task::{Task, TaskStatus};
use crate::Taskmanager::{TaskManager,SortOrder};
use std::io::{self, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use rusqlite::Result;
use crate::dbThread::{db_thread, DbMessage};
use uuid::Uuid;

pub struct ViewModel {
    task_manager: Arc<Mutex<TaskManager>>,
    db_thread_handle: thread::JoinHandle<()>,
    db_tx: mpsc::Sender<DbMessage>,
}

impl ViewModel {
    pub fn new() -> Result<Self, rusqlite::Error> {
        // Initialize the task manager
        let task_manager = Arc::new(Mutex::new(TaskManager::new()?));
        
        // Create a channel for communication between the main thread and the db thread
        let (db_tx, db_rx) = mpsc::channel();
        
        // Clone the task manager for the database thread
        let db_task_manager = task_manager.clone();

         // Spawn the db_thread
        let db_thread_handle = thread::spawn(move || {
            let conn = rusqlite::Connection::open("tasks.db").expect("Error opening the database");
            let task_manager = Arc::new(Mutex::new(TaskManager::new().unwrap()));
            db_thread(db_rx, conn);
        });

        Ok(ViewModel {
            task_manager,
            db_thread_handle,
            db_tx,
        })
    }

    pub fn add_task(&mut self, title: String,description: String, date: String) -> Uuid  {
        self.task_manager.lock().unwrap().add_task(title,description,date,&self.db_tx)
    }

    pub fn update_task_status(&mut self, task_id: Uuid, status: TaskStatus) -> bool {
        self.task_manager.lock().unwrap().update_task_status(task_id,status,&self.db_tx)
    }

    pub fn undo(&mut self) -> bool {
        self.task_manager.lock().unwrap().undo(&self.db_tx)
    }

    pub fn redo(&mut self) -> bool {
        self.task_manager.lock().unwrap().redo(&self.db_tx)
    }

    pub fn delete_task(&mut self, task_id: Uuid) -> bool {
        self.task_manager.lock().unwrap().delete_task(task_id,&self.db_tx)
    }

    pub fn list_tasks(&self) -> Option<Vec<Task>> {

        let task_list = self.task_manager.lock().unwrap().list_tasks();
        if task_list.len()>0 {
            return Some(task_list);
        }
        None
    }

    pub fn search_tasks(&self, keyword: &str) -> Option<Vec<Task>> {
        let task_list = self.task_manager.lock().unwrap().search_tasks(keyword);
        if task_list.len()>0 {
            return Some(task_list);
        }
        None
    }

    pub fn sort_tasks(&self,order:SortOrder ) {
        self.task_manager.lock().unwrap().sort_tasks_by_date(order);
    }
}

