
use crate::TaskManager;
use crate::task::Task;
use std::error::Error;

pub struct TaskViewModel {
    task_manager: TaskManager,
}

impl TaskViewModel {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let task_manager = TaskManager::new()?;
        Ok(TaskViewModel { task_manager })
    }

    // Add ViewModel functions to interact with the Model and provide data to the View
    pub fn add_task(&mut self, title: String, description: String, date: String) -> Result<(), Box<dyn std::error::Error>> {
        self.task_manager.add_task(title, description, date)?;
        Ok(())
    }

    pub fn mark_task_done(&mut self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.task_manager.mark_task_done(task_id)?;
        Ok(())
    }

    pub fn delete_task(&mut self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.task_manager.delete_task(task_id)?;
        Ok(())
    }

    pub fn sort_by_due_date(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.task_manager.sort_by_due_date()?;
        Ok(())
    }

    pub fn search_tasks(&mut self,query: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        self.task_manager.search_tasks(query).map_err(|err| Box::new(err) as Box<dyn Error>)
       // self.task_manager.search_tasks(query)?;
        //Ok(())
    }

    pub fn mark_task_undone(&mut self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.task_manager.mark_task_undone(task_id)?;
        Ok(())
    }

    pub fn archive_task(&mut self, task_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.task_manager.archive_task(task_id)?;
        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), String> {
        match self.task_manager.undo() {
            Ok(_) => Ok(()),
            Err(msg) => Err(msg)
        }
        
    }

    pub fn redo(&mut self) -> Result<(), String> {
        match self.task_manager.redo() {
            Ok(_) => Ok(()),
            Err(msg) => Err(msg)
        }
    }

    pub fn list_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        self.task_manager.list_tasks().map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    // Add other ViewModel functions as needed
}
