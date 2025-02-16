use crate::Source;
use crate::schema::sources;
use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_query;
use dotenvy::dotenv;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::Once;
use std::vec;
pub type DbPool = Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;
pub struct Database {
    pub pool: DbPool,
}
// 初始化数据库连接池
pub fn init_pool() -> DbPool {
    // 从 .env 文件加载环境变量
    dotenv().ok();

    // 获取数据库 URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 创建连接管理器
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    // 创建连接池并用 Arc 包装
    Arc::new(
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool"),
    )
}
/// 从连接池中获取一个连接
// pub fn get_conn(pool: &DbPool) -> PooledConnection {
//     pool.get().expect("Failed to get connection from pool")
// }
impl Database {
    pub fn default() -> Database {
        Database { pool: init_pool() }
    }
    /// 从连接池中获取一个连接
    pub fn get_conn(&self) -> PooledConnection {
        self.pool.get().expect("Failed to get connection from pool")
    }
    pub fn init_database(&mut self, pool: &DbPool) {
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            let pool = init_pool();
            let mut conn = self.get_conn();

            // 创建表（如果不存在）
            diesel::sql_query(
                r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL
            );
            "#,
            )
            .execute(&mut conn)
            .expect("Failed to create table");
            println!("Database initialized successfully!");
        });
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

    pub fn get_sources_collection(&self) {
        let mut conn = self.get_conn();
        let s = diesel::sql_query(
            r#"
       SELECT * FROM Sources ORDER BY child_order;
        "#,
        )
        .execute(&mut conn)
        // .load::<Source>(&mut self.conn)
        .expect("Failed to get Sources");
        println!("s:{:?}", s);
        println!("Database initialized successfully!");
    }
}
