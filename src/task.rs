use actix_web::{get, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub const APPLICATION_JSON: &str = "application/json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub results: Vec<T>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub status: String,
}

pub type Tasks = Response<Task>;

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            title,
            status: "Uncompleted".to_string(),
        }
    }
}

#[get("/tasks")]
pub async fn list() -> HttpResponse {

    //Sample task data
    let task1 = Task {
        id: "1".to_string(),
        created_at: Utc::now(),
        title: "Task 1".to_string(),
        status: "Uncompleted".to_string(),
    };

    let task2 = Task {
        id: "2".to_string(),
        created_at: Utc::now(),
        title: "Task 2".to_string(),
        status: "Doing".to_string(),
    };

    let tasks: Tasks = Response {
        results: vec![task1, task2],
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tasks)
}
