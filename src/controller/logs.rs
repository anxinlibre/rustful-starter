use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::bootstrap::db_pool;
use crate::model::logs as log_model;
use crate::data::logs as log_data;

pub fn routes() -> Vec<rocket::Route> {
    routes![list,info]
}

#[get("/")]
pub async fn list(
    conn: Connection<db_pool::Db>,
) -> Result<Json<Vec<log_model::Model>>, Status> {
    let log_list = log_data::get_all_logs(conn);
    Ok(Json(
        log_list.await
    ))
}

#[get("/<id>")]
pub async fn info(
    conn: Connection<db_pool::Db>, 
    id: i32,
) -> Result<Json<Option<log_model::Model>>, Status> {
    let log = log_data::get_log_by_id(conn, id);
    Ok(Json(
        log.await
    ))
}
