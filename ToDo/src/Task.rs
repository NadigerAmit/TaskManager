use uuid::Uuid;   //https://docs.rs/uuid/0.8.1/uuid/index.html#dependencies


// Define the structure to represent a task
#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub date: String, // We can use a proper Date/Time type here
}

// Enum to represent task status
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    Done,
    Archived,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TaskStatus::Todo => write!(f,"ToDo"),
            TaskStatus::Done => write!(f,"Done"),
            TaskStatus::Archived => write!(f,"Archived"),
        }
    }
}

/*
fn generate_short_id(uuid: Uuid) -> String {
    let mut hasher = Sha256::new();
    hasher.update(uuid.as_bytes());
    let hash_result = hasher.finalize();

    // Take the first 4 bytes (32 bits) of the hash and convert it to a string
    let short_id: String = format!("{:x}", &hash_result[..4]);
    short_id
}
*/

impl Task {
    pub fn new(title: String,description: String, date: String) -> Self {
        Task {
            id:Uuid::new_v4(),
            title,
            description,
            status: TaskStatus::Todo,
            date,
        }
    }
}
