
use crate::task::Task;

use rusqlite::{Connection, Result};
use std::io;

// Task Manager
pub struct TaskManager {
    conn: Connection,
    undo_stack: Vec<Vec<Task>>,
    redo_stack: Vec<Vec<Task>>,
}

impl TaskManager {
     // TaskManager constructor
     pub fn new() -> Result<Self> {
        let conn = Connection::open("tasks.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                 id INTEGER PRIMARY KEY,
                 title TEXT NOT NULL,
                 description TEXT,
                 date TEXT NOT NULL,
                 status TEXT NOT NULL
             )",
            [],
        )?;
        Ok(TaskManager {
            conn,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
     }

     // Function to add a new task to the database
     pub fn add_task(&mut self, title: String, description: String, date: String) -> Result<()> {
        self.conn.execute( "INSERT INTO tasks (title, description, date, status) VALUES (?1, ?2, ?3, ?4)",
        &[&title, &description, &date, "Undone"],
        )?;
        self.update_undo_stack()?;
        Ok(())
     }

     // Function to mark a task as done in the database
    pub fn mark_task_done(&mut self, task_id: i32) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET status = 'Done' WHERE id = ?1",
            &[&task_id],
        )?;
        self.update_undo_stack()?;
        Ok(())
    }

    pub fn mark_task_undone(&mut self, task_id: i32) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET status = 'Undone' WHERE id = ?1",
            &[&task_id],
        )?;
        self.update_undo_stack()?;
        Ok(())
    }

    pub fn archive_task(&mut self, task_id: i32) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET status = 'Archived' WHERE id = ?1",
            &[&task_id],
        )?;
        self.update_undo_stack()?;
        Ok(())
    }

    pub fn delete_task(&mut self, task_id: i32) -> Result<()> {
        self.conn.execute(
            "DELETE FROM tasks WHERE id = ?1",
            &[&task_id],
        )?;
        self.update_undo_stack()?;
        Ok(())
    }

    // Function to undo the last action
     pub fn undo(&mut self) -> Result<(), String> {
        if let Some(prev_state) = self.undo_stack.pop() {
           // self.redo_stack.push(self.get_all_tasks()?);
            let _ = self.restore_task_state(prev_state);
            Ok(())
        } else {
            Err("Nothing to undo.".to_string())
        }
    }

    // Function to redo the last undone action
    pub fn redo(&mut self) -> Result<(), String> {
        if let Some(prev_state) = self.redo_stack.pop() {
            // self.undo_stack.push(self.get_all_tasks()?);
            let _= self.restore_task_state(prev_state);
            Ok(())
        } else {
            Err("Nothing to redo.".to_string())
        }
    }

     // Function to sort tasks by due date (ascending)
     pub fn sort_by_due_date(&mut self) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks ORDER BY date")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                date: row.get(3)?,
                status: row.get(4)?,
            })
        })?;

        let mut sorted_tasks = Vec::new();
        for task in task_iter {
            sorted_tasks.push(task?);
        }

        //self.restore_task_state(sorted_tasks)?;
        Ok(())
    }

    // Function to search tasks by title or description
    pub fn search_tasks(&self, query: &str) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM tasks WHERE title LIKE ?1 OR description LIKE ?1")?;
        let task_iter = stmt.query_map(&[&format!("%{}%", query)], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                date: row.get(3)?,
                status: row.get(4)?,
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }
    
    pub fn list_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks WHERE status NOT IN ('Archived')")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                date: row.get(3)?,
                status: row.get(4)?,
            })
        })?;
        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    // Private function to update the undo stack
    pub fn update_undo_stack(&mut self) -> Result<()> {
        let tasks = self.get_all_tasks()?;
        self.undo_stack.push(tasks);
        self.redo_stack.clear();
        Ok(())
    }

    // Private function to restore the task state from a given state
    fn restore_task_state(&mut self, tasks: Vec<Task>) -> Result<()> {
        self.conn.execute("DELETE FROM tasks", [])?;
        for task in &tasks {
           /* 
            self.conn.execute(
                "INSERT INTO tasks (id, title, description, date, status) VALUES (?1, ?2, ?3, ?4, ?5)",
                &[&task.id, &task.title, &task.description, &task.date, &task.status],
            )?;
            */
            self.conn.execute( "INSERT INTO tasks (title, description, date, status) VALUES (?1, ?2, ?3, ?4)",
            &[&task.title, &task.description, &task.date, &task.status],
            )?;
        }
        Ok(())
    }

    // Private function to get all tasks from the database
    fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let tasks = self.list_tasks()?;
        Ok(tasks)
    }
}