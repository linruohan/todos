#![allow(unused)]
use chrono_humanize::{Accuracy, HumanTime};

pub mod enums;
pub mod objects;
pub mod services;
pub mod utils;
use anyhow::{Ok, Result};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, Utc};
pub(crate) use objects::{Attachment, BaseObject, Item, Label, Project, Reminder, Section, Source};
pub(crate) use services::Database;
pub(crate) use services::models::schema;

fn main() -> Result<()> {
    let db = Database::default();
    db.get_sources_collection();
    // todo
    // let _ = Todo::new("title".to_owned(), "content".to_owned());
    // let todos = Todo::todos();
    let today = Local::now().naive_local();
    let start_of_week = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let end_of_week = start_of_week + Duration::days(6);
    println!("{}", today);
    println!("{}", today.weekday());
    println!("{}", today.weekday().num_days_from_monday());
    println!("{}", start_of_week);
    println!("{}", end_of_week);
    let human_time = HumanTime::from(Duration::seconds(3116000));
    println!(
        "{}",
        human_time.to_text_en(Accuracy::Rough, chrono_humanize::Tense::Past)
    );

    Ok(())
}

// use std::thread;

// fn main() {
//     // 初始化连接池并用 Arc 包装
//     let pool: DbPool = init_pool();

//     // 克隆 Arc 以共享连接池
//     let pool_clone = Arc::clone(&pool);

//     // 在另一个线程中插入用户
//     let handle = thread::spawn(move || {
//         let new_user = NewUser {
//             name: "Bob".to_string(),
//             email: "bob@example.com".to_string(),
//         };
//         insert_user(&pool_clone, new_user).expect("Failed to insert user");
//     });

//     // 在主线程中查询用户
//     let users = get_all_users(&pool).expect("Failed to load users");
//     for user in users {
//         println!("User: {} ({})", user.name, user.email);
//     }

//     // 等待子线程完成
//     handle.join().unwrap();
// }
