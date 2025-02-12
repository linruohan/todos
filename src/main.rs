#![allow(unused)]
use chrono_humanize::{Accuracy, HumanTime};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{SqliteConnection, r2d2};
use dotenvy::dotenv;
use models::todo::imp::Todo;
use std::env;
use std::sync::Arc;
pub mod enums;
pub mod models;
pub mod objects;
pub mod utils;
use anyhow::{Ok, Result};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, Utc};
pub(crate) use objects::BaseObject;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("SQLITE_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type DbPool = Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>;
pub fn get_db() -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.");
    let db = Arc::new(pool).get().map_err(|e| anyhow::anyhow!(e))?;
    Ok(db)
}

fn main() -> Result<()> {
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
