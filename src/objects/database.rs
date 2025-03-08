use crate::Source;
use crate::schema::sources;
use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_query;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::Once;
use std::vec;

use super::Section;
use super::{Attachment, Item, Label, Project, Reminder};
pub type DbPool = Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>;
// 定义全局的数据库连接池
pub static DB_POOL: OnceCell<DbPool> = OnceCell::new();
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;
pub struct Database {}
fn get_db_pool() -> &'static DbPool {
    DB_POOL.get_or_init(|| {
        dotenv().ok(); // 加载 .env 文件
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create database pool");
        Arc::new(pool)
    })
}

/// 从连接池中获取一个连接
// pub fn get_conn(pool: &DbPool) -> PooledConnection {
//     pool.get().expect("Failed to get connection from pool")
// }
impl Database {
    pub fn default() -> Database {
        Self {}
    }
    /// 从连接池中获取一个连接
    pub fn get_conn(&self) -> PooledConnection {
        let pool = get_db_pool().clone();
        pool.get().expect("Failed to get connection from pool")
    }

    pub fn clear_database(&self, pool: DbPool) -> Result<(), String> {
        // 1 关闭连接池
        let _ = drop(pool);
        // 2 删除文件
        // 2.1 获取数据库 URL
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // 2.2 从数据库 URL 中提取文件路径
        let file_path = database_url
            .strip_prefix("sqlite://")
            .ok_or("Invalid database URL format")?;

        // 2.3 检查文件是否存在
        if !Path::new(file_path).exists() {
            return Err("Database file does not exist".to_string());
        }
        // 3.4删除文件
        fs::remove_file(file_path).map_err(|e| e.to_string())?;

        println!("Database file deleted: {}", file_path);
        Ok(())
    }

    pub fn get_sources_collection(&self) -> Vec<Source> {
        let mut conn = self.get_conn();
        diesel::sql_query(
            r#"
       SELECT * FROM Sources ORDER BY child_order;
        "#,
        )
        // .execute(&mut conn)
        .load::<Source>(&mut conn)
        .expect("Failed to get Sources")
    }
    pub fn get_items_collection(&self) -> Vec<Item> {
        let mut conn = self.get_conn();
        diesel::sql_query(
            r#"
           SELECT * FROM items WHERE is_deleted = 0;
             "#,
        )
        // .execute(&mut conn)
        .load::<Item>(&mut conn)
        .expect("Failed to get Sources")
    }
    pub fn get_attachments_collection(&self) -> Vec<Attachment> {
        let mut conn = self.get_conn();
        diesel::sql_query(
            r#"
          SELECT * FROM attachments;
             "#,
        )
        // .execute(&mut conn)
        .load::<Attachment>(&mut conn)
        .expect("Failed to get Sources")
    }
    pub fn delete_attachment(&self, attachment: Attachment) -> bool {
        let mut conn = self.get_conn();
        let ret = diesel::sql_query(
            r#"
         DELETE FROM Attachments WHERE id=$id;
             "#,
        )
        .execute(&mut conn);
        if !ret.is_ok() {
            return false;
        }
        return true;
    }

    pub(crate) fn get_projects_collection(&self) -> Vec<Project> {
        let mut conn = self.get_conn();
        diesel::sql_query(
            r#"
          SELECT * FROM Projects WHERE is_deleted = 0 ORDER BY child_order;
             "#,
        )
        .load::<Project>(&mut conn)
        .expect("Failed to get Project")
    }

    pub(crate) fn get_labels_collection(&self) -> Vec<Label> {
        let mut conn = self.get_conn();
        diesel::sql_query(
            r#"
          SELECT * FROM Labels;
             "#,
        )
        .load::<Label>(&mut conn)
        .expect("Failed to get Label")
    }

    pub(crate) fn get_reminders_collection(&self) -> Vec<Reminder> {
        let mut conn = self.get_conn();
        diesel::sql_query(
            r#"
          SELECT id, item_id, type, due, mm_offset FROM Reminders;
             "#,
        )
        .load::<Reminder>(&mut conn)
        .expect("Failed to get Label")
    }

    pub(crate) fn get_sections_collection(&self) -> Vec<Section> {
        let mut conn = self.get_conn();
        diesel::sql_query(
            r#"
          SELECT * FROM Sections WHERE is_deleted = 0;
             "#,
        )
        .load::<Section>(&mut conn)
        .expect("Failed to get Section")
    }
}
