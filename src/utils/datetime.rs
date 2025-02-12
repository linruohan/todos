use std::str::FromStr;

use crate::{enums::RecurrencyType, objects::DueDate};
use anyhow::Result;
use chrono::{
    Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, ParseError, Timelike,
};
use chrono_humanize::{Accuracy, HumanTime};
use diesel::{dsl::date, sql_types::Json};
const EMPTY_DATETIME: NaiveDateTime = chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc();
pub struct DateTime {}
impl DateTime {
    pub fn get_todoist_datetime(&self, date: String) -> Result<NaiveDateTime, ParseError> {
        // YYYY-MM-DD
        if date.len() == 10 {
            return NaiveDateTime::parse_from_str(
                format!("{} 00:00:00", date).as_str(),
                "%Y-%m-%d %H:%M:%S",
            );
        } else {
            // YYYY-MM-DDTHH:MM:SS
            return NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S");
        }
    }

    pub fn get_relative_date_from_date(&self, datetime: NaiveDateTime) -> String {
        let mut returned = "";
        let format_str = self.get_default_date_format_from_date(datetime).clone();
        if (self.is_today(datetime)) {
            returned = "Today";
        } else if self.is_tomorrow(datetime) {
            returned = "Tomorrow";
        } else if self.is_yesterday(datetime) {
            returned = "Yesterday";
        } else {
            returned = format_str.as_str();
        }
        if self.has_time(datetime) {
            return format!(
                "{} {}",
                returned,
                datetime.format(self.get_default_time_format())
            );
        }
        return returned.to_string();
    }

    pub fn get_relative_datetime(&self, datetime: NaiveDateTime) -> String {
        let now = Local::now().naive_local();

        let human_time = HumanTime::from(now - datetime);
        human_time.to_text_en(Accuracy::Rough, chrono_humanize::Tense::Past)
    }

    pub fn days_left(&self, datetime: NaiveDateTime, show_today: bool) -> String {
        let days = (datetime - Local::now().naive_local()).num_days();
        if self.is_today(datetime) {
            return if show_today {
                "Today".to_string()
            } else {
                "".to_string()
            };
        } else if self.is_overdue(datetime) {
            return format!(
                "{} {} ago",
                (days * -1).to_string(),
                if days > 1 { "days" } else { "day" }
            );
        } else {
            return format!(
                "{} {} left",
                (days + 1).to_string(),
                if days > 1 { "days" } else { "day" }
            );
        }
        return "".to_string();
    }

    pub fn get_default_time_format(&self) -> &str {
        if self.is_clock_format_12h() {
            "%I:%M"
        } else {
            "%H:%M"
        }
    }

    pub fn is_clock_format_12h(&self) -> bool {
        // return Services.Settings.get_default ().settings.get_string ("clock-format").contains ("12h");
        false
    }

    pub fn is_yesterday(&self, date: NaiveDateTime) -> bool {
        return self.is_same_day(date, Local::now().naive_local() - Duration::days(1));
    }

    pub fn is_same_day(&self, day1: NaiveDateTime, day2: NaiveDateTime) -> bool {
        return day1.year() == day2.year() && day1.day() == day2.day();
    }

    pub fn is_overdue(&self, date: NaiveDateTime) -> bool {
        // date在今天之前，说明是过期了
        if date < Local::now().naive_local() {
            return true;
        }
        return false;
    }

    pub fn get_calendar_icon(&self, date: NaiveDateTime) -> &str {
        if self.is_today(date) {
            return "planner-today";
        }
        return "planner-scheduled";
    }

    pub fn parse_todoist_recurrency(duedate: DueDate, object: Json) {
        // if (object.has_member("lang") && object.get_string_member("lang") != "en") {
        //     duedate.recurrency_supported = false;
        // }
    }

    pub fn has_time(&self, datetime: NaiveDateTime) -> bool {
        if (datetime == EMPTY_DATETIME) {
            return false;
        }
        if (datetime.hour() == 0 && datetime.minute() == 0 && datetime.second() == 0) {
            return false;
        }
        return true;
    }

    pub fn is_today(&self, date: NaiveDateTime) -> bool {
        if date == EMPTY_DATETIME {
            return false;
        }
        return self.is_same_day(date, Local::now().naive_local());
    }

    pub fn is_tomorrow(&self, date: NaiveDateTime) -> bool {
        if date == EMPTY_DATETIME {
            return false;
        }
        return self.is_same_day(date, Local::now().naive_local() + Duration::days(1));
    }

    pub fn is_next_week(&self, date: NaiveDateTime) -> bool {
        if date == EMPTY_DATETIME {
            return false;
        }
        return self.is_same_day(date, Local::now().naive_local() + Duration::days(7));
    }

    pub fn get_date_from_string(&self, date: String) -> NaiveDateTime {
        NaiveDateTime::from_str(&date).unwrap()
    }

    // pub static void recurrence_to_due (ICal.Recurrence recurrence, Objects.DueDate due) {
    //     due.is_recurring = true;

    //     ICal.RecurrenceFrequency freq = recurrence.get_freq ();
    //     if (freq == ICal.RecurrenceFrequency.MINUTELY_RECURRENCE) {
    //         due.recurrency_type = RecurrencyType.MINUTELY;
    //     } else if (freq == ICal.RecurrenceFrequency.HOURLY_RECURRENCE) {
    //         due.recurrency_type = RecurrencyType.HOURLY;
    //     } else if (freq == ICal.RecurrenceFrequency.DAILY_RECURRENCE) {
    //         due.recurrency_type = RecurrencyType.EVERY_DAY;
    //     } else if (freq == ICal.RecurrenceFrequency.WEEKLY_RECURRENCE) {
    //         due.recurrency_type = RecurrencyType.EVERY_WEEK;
    //     } else if (freq == ICal.RecurrenceFrequency.MONTHLY_RECURRENCE) {
    //         due.recurrency_type = RecurrencyType.EVERY_MONTH;
    //     } else if (freq == ICal.RecurrenceFrequency.YEARLY_RECURRENCE) {
    //         due.recurrency_type = RecurrencyType.EVERY_YEAR;
    //     }

    //     short interval = recurrence.get_interval ();
    //     due.recurrency_interval = (int) interval;

    //     int count = recurrence.get_count ();
    //     due.recurrency_count = count;

    //     ICal.Time until = recurrence.get_until ();
    //     if (!until.is_null_time ()) {
    //         due.recurrency_end = ical_to_date_time_local (until).to_string ();
    //     }

    //     if (due.recurrency_type == RecurrencyType.EVERY_WEEK) {
    //         string recurrency_weeks = "";
    //         GLib.Array<short> day_array = recurrence.get_by_day_array ();

    //         if (check_by_day ("1", day_array)) {
    //             recurrency_weeks += "7,";
    //         }

    //         if (check_by_day ("2", day_array)) {
    //             recurrency_weeks += "1,";
    //         }

    //         if (check_by_day ("3", day_array)) {
    //             recurrency_weeks += "2,";
    //         }

    //         if (check_by_day ("4", day_array)) {
    //             recurrency_weeks += "3,";
    //         }

    //         if (check_by_day ("5", day_array)) {
    //             recurrency_weeks += "4,";
    //         }

    //         if (check_by_day ("6", day_array)) {
    //             recurrency_weeks += "5,";
    //         }

    //         if (check_by_day ("7", day_array)) {
    //             recurrency_weeks += "6,";
    //         }

    //         if (recurrency_weeks.split (",").length > 0) {
    //             recurrency_weeks.slice (0, -1);
    //         }

    //         due.recurrency_weeks = recurrency_weeks;
    //     }
    // }

    fn check_by_day(&self, day: String, day_array: Vec<NaiveDateTime>) -> bool {
        for day1 in day_array.iter() {
            if day1.to_string() == day {
                return true;
            }
        }
        return false;
    }

    pub fn format_date(&self, format_date: NaiveDate) -> NaiveDateTime {
        NaiveDateTime::new(format_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    }

    pub fn is_this_week(&self, date: NaiveDateTime) -> bool {
        let current_date = date.date();
        let start_of_week =
            current_date - Duration::days(current_date.weekday().num_days_from_monday() as i64);
        let end_of_week = start_of_week + Duration::days(6);

        if date >= self.format_date(start_of_week) && date <= self.format_date(end_of_week) {
            return true;
        }
        return false;
    }

    pub fn is_next_x_week(&self, date: NaiveDateTime, days: i64) -> bool {
        let current_date = Local::now().naive_local();
        let end_date = current_date + Duration::days(days);

        if date >= current_date && date <= end_date {
            return true;
        }
        return false;
    }

    pub fn is_this_month(&self, date: NaiveDateTime) -> bool {
        let current_date = Local::now().naive_local();
        return current_date.month() == date.month() && current_date.year() == date.year();
    }

    // pub static GLib.DateTime next_recurrency (GLib.DateTime datetime, Objects.DueDate duedate) {
    //     GLib.DateTime returned = datetime;

    //     if (duedate.recurrency_type == RecurrencyType.MINUTELY) {
    //         returned = returned.add_minutes (duedate.recurrency_interval);
    //     } else if (duedate.recurrency_type == RecurrencyType.HOURLY) {
    //         returned = returned.add_hours (duedate.recurrency_interval);
    //     } else if (duedate.recurrency_type == RecurrencyType.EVERY_DAY) {
    //         returned = returned.add_days (duedate.recurrency_interval);
    //     } else if (duedate.recurrency_type == RecurrencyType.EVERY_WEEK) {
    //         if (duedate.recurrency_weeks == "") {
    //             returned = returned.add_days (duedate.recurrency_interval * 7);
    //         } else {
    //             returned = next_recurrency_week (datetime, duedate, true);
    //         }
    //     } else if (duedate.recurrency_type == RecurrencyType.EVERY_MONTH) {
    //         returned = returned.add_months (duedate.recurrency_interval);
    //     } else if (duedate.recurrency_type == RecurrencyType.EVERY_YEAR) {
    //         returned = returned.add_years (duedate.recurrency_interval);
    //     }

    //     return returned;
    // }

    pub fn get_next_day_of_week_from_recurrency_week(
        datetime: NaiveDateTime,
        duedate: DueDate,
    ) -> Option<i32> {
        let weeks: Vec<&str> = duedate.recurrency_weeks.split(",").collect();
        let day_of_week = datetime.weekday().num_days_from_monday() as i32;
        for week in &weeks {
            if let Ok(week) = week.parse::<i32>() {
                if week > day_of_week {
                    return Some(week);
                }
            } else {
                eprintln!("failed to parse week:{week}");
            }
        }

        if let Some(first_week_str) = weeks.get(0) {
            if let Ok(first_week) = first_week_str.parse::<i32>() {
                return Some(first_week);
            }
        }
        None
    }

    // pub static GLib.DateTime next_recurrency_week (GLib.DateTime datetime, Objects.DueDate duedate, bool user = false) {
    //     string[] weeks = duedate.recurrency_weeks.split (","); // [1, 2, 3]
    //     int day_of_week = datetime.get_day_of_week (); // 2
    //     int days = 0;
    //     int next_day = 0;
    //     int index = 0;
    //     int recurrency_interval = 0;

    //     for (int i = 0; i < weeks.length ; i++) {
    //         if (day_of_week < int.parse (weeks[i])) {
    //             index = i;
    //             break;
    //         }
    //     }

    //     next_day = int.parse (weeks[index]);

    //     if (day_of_week < next_day) {
    //         days = next_day - day_of_week;
    //     } else {
    //         days = 7 - (day_of_week - next_day);
    //     }

    //     if (user && index == 0) {
    //         recurrency_interval = (duedate.recurrency_interval - 1) * 7;
    //     }

    //     return datetime.add_days (days).add_days (recurrency_interval);
    // }

    pub fn get_recurrency_weeks(
        recurrency_type: RecurrencyType,
        recurrency_interval: i32,
        recurrency_weeks: String,
        end: String,
    ) -> String {
        let mut returned = recurrency_type.to_friendly_string(recurrency_interval);

        if recurrency_type == RecurrencyType::EveryWeek && !recurrency_weeks.is_empty() {
            let mut weeks = String::new();

            if recurrency_weeks.contains("1") {
                weeks.push_str("Mo,");
            }

            if recurrency_weeks.contains("2") {
                weeks.push_str("Tu,");
            }

            if recurrency_weeks.contains("3") {
                weeks.push_str("We,");
            }

            if recurrency_weeks.contains("4") {
                weeks.push_str("Th,");
            }

            if recurrency_weeks.contains("5") {
                weeks.push_str("Fr,");
            }

            if recurrency_weeks.contains("6") {
                weeks.push_str("Sa,");
            }

            if recurrency_weeks.contains("7") {
                weeks.push_str("Su,");
            }

            if !weeks.is_empty() {
                weeks.pop(); // Remove the trailing comma
            }

            returned = format!("{} ({})", returned, weeks);
        }
        format!("{} {}", returned, end)
    }

    pub fn get_today_format_date(&self) -> NaiveDateTime {
        return self.get_date_only(Local::now().naive_local());
    }

    pub fn get_date_only(&self, date: NaiveDateTime) -> NaiveDateTime {
        NaiveDateTime::new(date.date(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    }

    pub fn get_default_date_format_from_date(&self, date: NaiveDateTime) -> String {
        let year = if date.year() == Local::now().year() {
            ""
        } else {
            "%Y-"
        };
        format!("{}%m-%d %p", year)
    }

    pub fn get_todoist_datetime_format(&self, date: NaiveDateTime) -> String {
        if (self.has_time(date)) {
            return format!(
                "{}T{}",
                date.format("%F").to_string(),
                date.format("%T").to_string()
            );
        } else {
            return date.format("%F").to_string();
        }

        return "".to_string();
    }

    pub fn has_time_from_string(&self, date: NaiveDateTime) -> bool {
        return self.has_time(date);
    }

    pub fn get_days_of_month(index: i32, year_nav: i32) -> i32 {
        if ((index == 1)
            || (index == 3)
            || (index == 5)
            || (index == 7)
            || (index == 8)
            || (index == 10)
            || (index == 12))
        {
            return 31;
        } else {
            if (index == 2) {
                if (year_nav % 4 == 0) {
                    return 29;
                } else {
                    return 28;
                }
            } else {
                return 30;
            }
        }
    }

    pub fn get_start_of_month(&self, date: NaiveDateTime) -> NaiveDateTime {
        if date == EMPTY_DATETIME {
            let date = Local::now().naive_local();
        }
        let date1 = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();
        let time1 = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        NaiveDateTime::new(date1, time1)
    }

    pub fn is_current_month(&self, date: NaiveDateTime) -> bool {
        let now = Local::now().naive_local();
        if (date.year() == now.year()) {
            if (date.month() == now.month()) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    // /**
    //  * Converts the given ICal.Time to a GLib.DateTime, represented in the
    //  * system timezone.
    //  *
    //  * All timezone information in the original @date is lost. However, the
    //  * {@link GLib.TimeZone} contained in the resulting DateTime is correct,
    //  * since there is a well-defined local timezone between both libical and
    //  * GLib.
    //  */
    //  pub static DateTime ical_to_date_time_local (ICal.Time date) {
    //     assert (!date.is_null_time ());
    //     let converted = ical_convert_to_local (date);
    //     int year, month, day, hour, minute, second;
    //     converted.get_date (out year, out month, out day);
    //     converted.get_time (out hour, out minute, out second);
    //     return new DateTime.local (year, month,
    //         day, hour, minute, second);
    // }

    // /** Converts the given ICal.Time to the local (or system) timezone */
    // pub static ICal.Time ical_convert_to_local (ICal.Time time) {
    //     let system_tz = ECal.util_get_system_timezone ();
    //     return time.convert_to_zone (system_tz);
    // }

    // /**
    //  * Converts two DateTimes representing a date and a time to one TimeType.
    //  *
    //  * The first contains the date; its time settings are ignored. The second
    //  * one contains the time itself; its date settings are ignored. If the time
    //  * is `null`, the resulting TimeType is of `DATE` type; if it is given, the
    //  * TimeType is of `DATE-TIME` type.
    //  *
    //  * This also accepts an optional `timezone` argument. If it is given a
    //  * timezone, the resulting TimeType will be relative to the given timezone.
    //  * If it is `null`, the resulting TimeType will be "floating" with no
    //  * timezone. If the argument is not given, it will default to the system
    //  * timezone.
    //  */
    //  pub static ICal.Time datetimes_to_icaltime (GLib.DateTime date, GLib.DateTime? time_local,
    //     ICal.Timezone? timezone = ECal.util_get_system_timezone ().copy ()) {
    //     let result = new ICal.Time.from_day_of_year (date.get_day_of_year (), date.get_year ());

    //     // Check if it's a date. If so, set is_date to true and fix the time to be sure.
    //     // If it's not a date, first thing set is_date to false.
    //     // Then, set the timezone.
    //     // Then, set the time.
    //     if (time_local == null) {
    //         // Date type: ensure that everything corresponds to a date
    //         result.set_is_date (true);
    //         // result.set_time (0, 0, 0);
    //     } else {
    //         // Includes time
    //         // Set is_date first (otherwise timezone won't change)
    //         result.set_is_date (false);

    //         // Set timezone for the time to be relative to
    //         // (doesn't affect DATE-type times)
    //         result.set_timezone (timezone);

    //         // Set the time with the updated time zone
    //         result.set_time (time_local.get_hour (), time_local.get_minute (), time_local.get_second ());
    //     }

    //     return result;
    // }

    // pub static string get_markdown_format_date (Objects.Item item) {
    //     if (!item.has_due) {
    //         return " ";
    //     }

    //     return " (" + get_relative_date_from_date (item.due.datetime) + ") ";
    // }

    // pub static GLib.DateTime get_datetime_no_seconds (GLib.DateTime date, GLib.DateTime? time = null) {
    //     return new DateTime.local (
    //         date.get_year (),
    //         date.get_month (),
    //         date.get_day_of_month (),
    //         time == null ? date.get_hour () : time.get_hour (),
    //         time == null ? date.get_minute () : time.get_minute (),
    //         0
    //     );
    // }
}
