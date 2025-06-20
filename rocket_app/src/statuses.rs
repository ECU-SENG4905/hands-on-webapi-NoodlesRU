use rocket::{serde::json::Json, State, get, post, put, delete};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use tasks_db_lib::models::{TaskStatus, NewTaskStatus};
use tasks_db_lib::crud::CrudOperations;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(rocket::serde::Deserialize)]
pub struct StatusInput {
    pub status_name: String,
}

#[get("/task_statuses")]
pub async fn get_task_statuses(pool: &State<DbPool>) -> Json<Vec<TaskStatus>> {
    let mut conn = pool.get().expect("db connection");
    let statuses = TaskStatus::read_all(&mut conn).unwrap_or_default();
    Json(statuses)
}

#[get("/task_statuses/<id>")]
pub async fn get_task_status(id: i32, pool: &State<DbPool>) -> Option<Json<TaskStatus>> {
    let mut conn = pool.get().ok()?;
    TaskStatus::read(&mut conn, id).ok().flatten().map(Json)
}

#[post("/task_statuses", data = "<status>")]
pub async fn create_task_status(pool: &State<DbPool>, status: Json<StatusInput>) -> Option<Json<TaskStatus>> {
    let mut conn = pool.get().ok()?;
    let new_status = NewTaskStatus {
        status_name: status.status_name.as_str(),
    };
    TaskStatus::create(&mut conn, new_status).ok().map(Json)
}

#[put("/task_statuses/<id>", data = "<status>")]
pub async fn update_task_status(id: i32, pool: &State<DbPool>, status: Json<StatusInput>) -> Option<Json<TaskStatus>> {
    let mut conn = pool.get().ok()?;
    let updated_status = NewTaskStatus {
        status_name: status.status_name.as_str(),
    };
    TaskStatus::update(&mut conn, id, updated_status).ok().map(Json)
}

#[delete("/task_statuses/<id>")]
pub async fn delete_task_status(id: i32, pool: &State<DbPool>) -> Option<Json<usize>> {
    let mut conn = pool.get().ok()?;
    TaskStatus::delete(&mut conn, id).ok().map(Json)
}
