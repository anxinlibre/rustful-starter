use crate::model::*;
use sea_orm::{DbConn, EntityTrait, Schema};

async fn create_table<E>(db: &DbConn, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let stmt = builder.build(Schema::create_table_from_entity(entity).if_not_exists());

    match db.execute(stmt).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}

// 初始化表结构
pub async fn create_tables(db: &DbConn) {
    create_table(db, Log).await;
}
