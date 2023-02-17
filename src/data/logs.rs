use rocket_db_pools::Connection;
use sea_orm::entity::*;


use crate::model::logs::{Entity as Log, Model};
use crate::bootstrap::db_pool;

// 获取所有记录
pub async fn get_all_logs(conn: Connection<db_pool::Db>) -> Vec<Model> {
    Log::find()
            .all(&conn)
            .await
            .expect("could not find logs")
            .into_iter()
            .collect::<Vec<Model>>()
}

// 根据id查找记录
pub async fn get_log_by_id(conn: Connection<db_pool::Db>, id:i32) -> Option<Model> {
    Log::find_by_id(id)
    .one(&conn)
    .await
    .expect("could not find logs")
}

