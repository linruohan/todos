use std::str::FromStr;

use crate::enums::{RecurrencyEndType, RecurrencyType};
use chrono::NaiveDateTime;
pub struct DueDate {
    pub date: String,
    pub timezone: String,
    pub recurrency_weeks: String,
    pub is_recurring: bool,
    pub recurrency_type: RecurrencyType,
    pub recurrency_interval: i32,
    pub recurrency_count: i32,
    pub recurrency_end: String,
    pub recurrency_supported: bool,
}
impl DueDate {
    pub fn datetime(&self) -> NaiveDateTime {
        NaiveDateTime::from_str(&self.date).unwrap()
    }
    pub fn set_datetime(&mut self, value: NaiveDateTime) {
        self.date = value.format("%Y-%m-%d %H:%M:%S").to_string();
    }
    pub fn end_datetime(&self) -> NaiveDateTime {
        NaiveDateTime::from_str(&self.recurrency_end).unwrap()
    }
    pub fn has_weeks(&self) -> bool {
        self.recurrency_weeks != ""
    }
    pub fn end_type(&self) -> RecurrencyEndType {
        if self.recurrency_end != "" {
            return RecurrencyEndType::OnDate;
        }
        if self.recurrency_count > 0 {
            return RecurrencyEndType::AFTER;
        }
        return RecurrencyEndType::NEVER;
    }
    pub fn is_recurrency_end(&self) -> bool {
        match self.end_type() {
            RecurrencyEndType::NEVER => false,
            // RecurrencyEndType::OnDate => {
            //     let next_recurrency=NaiveDateTime::
            // },
            RecurrencyEndType::AFTER => self.recurrency_count - 1 <= 0,
            RecurrencyEndType::OnDate => todo!(),
        }
    }
}
