use std::str::FromStr;

use anyhow::Result;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, ParseError};
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

    // pub fn get_relative_date_from_date ( datetime:NaiveDateTime) ->&str{
    //     if (is_today(datetime)) {
    //         returned = "Today";
    //     } else if (is_tomorrow(datetime)) {
    //         returned = _("Tomorrow");
    //     } else if (is_yesterday (datetime)) {
    //         returned = _("Yesterday");
    //     } else {
    //         returned = get_default_date_format_from_date (datetime);
    //     }

    //     if (has_time (datetime)) {
    //         returned = "%s %s".printf (returned, datetime.format (get_default_time_format ()));
    //     }

    //     return returned;
    // }

    // pub static string get_relative_datetime (GLib.DateTime datetime) {
    //     return Granite.DateTime.get_relative_datetime (datetime);
    // }

    // pub static string days_left (GLib.DateTime datetime, bool show_today = false) {
    //     string return_value = "";
    //     let days = datetime.difference (new GLib.DateTime.now_local ()) / TimeSpan.DAY;

    //     if (is_today (datetime)) {
    //         return_value = show_today ? _("Today") : "";
    //     } else if (is_overdue (datetime)) {
    //         return_value = _("%s %s ago".printf ((days * -1).to_string (), days > 1 ? _("days") : _("day")));
    //     } else {
    //         return_value = _("%s %s left".printf ((days + 1).to_string (), days > 1 ? _("days") : _("day")));
    //     }

    //     return return_value;
    // }

    // pub static string get_default_time_format () {
    //     return Granite.DateTime.get_default_time_format (
    //         is_clock_format_12h (), false
    //     );
    // }

    // pub static bool is_clock_format_12h () {
    //     return Services.Settings.get_default ().settings.get_string ("clock-format").contains ("12h");
    // }

    pub fn is_yesterday(&self, date: NaiveDateTime) -> bool {
        return self.is_same_day(date, Local::now().naive_local() - Duration::days(1));
    }

    pub fn is_same_day(&self, day1: NaiveDateTime, day2: NaiveDateTime) -> bool {
        return day1.year() == day2.year() && day1.day() == day2.day();
    }

    // pub static bool is_overdue (GLib.DateTime date) {
    //     if (get_date_only (date).compare (get_date_only (new DateTime.now_local ())) == -1) {
    //         return true;
    //     }

    //     return false;
    // }

    // pub static string get_calendar_icon (GLib.DateTime date) {
    //     if (is_today (date)) {
    //         return "planner-today";
    //     } else {
    //         return "planner-scheduled";
    //     }
    // }

    // pub static void parse_todoist_recurrency (Objects.DueDate duedate, Json.Object object) {
    //     if (object.has_member ("lang") && object.get_string_member ("lang") != "en") {
    //         duedate.recurrence_supported = false;
    //         return;
    //     }
    // }

    // pub static bool has_time (GLib.DateTime datetime) {
    //     if (datetime == null) {
    //         return false;
    //     }

    //     bool returned = true;

    //     if (datetime.get_hour () == 0 && datetime.get_minute () == 0 && datetime.get_second () == 0) {
    //         returned = false;
    //     }

    //     return returned;
    // }

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

    // pub static bool is_next_x_week (GLib.DateTime date, int days) {
    //     let current_date = new GLib.DateTime.now_local ();
    //     let end_date = current_date.add_days (days);

    //     if (date.compare (format_date (current_date)) >= 0 &&
    //         date.compare (format_date (end_date)) <= 0) {
    //         return true;
    //     }

    //     return false;
    // }

    // pub static bool is_this_month (GLib.DateTime date) {
    //     let current_date = new GLib.DateTime.now_local ();
    //     return current_date.get_month () == date.get_month () && current_date.get_year () == date.get_year ();
    // }

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

    // pub static int get_next_day_of_week_from_recurrency_week (GLib.DateTime datetime, Objects.DueDate duedate) {
    //     string[] weeks = duedate.recurrency_weeks.split (",");
    //     int day_of_week = datetime.get_day_of_week ();
    //     int index = 0;

    //     for (int i = 0; i < weeks.length ; i++) {
    //         if (day_of_week <= int.parse (weeks[i])) {
    //             index = i;
    //             break;
    //         }
    //     }

    //     if (index > weeks.length - 1) {
    //         index = 0;
    //     }

    //     return int.parse (weeks[index]);
    // }

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

    // pub static string get_recurrency_weeks (RecurrencyType recurrency_type, int recurrency_interval,
    //     string recurrency_weeks, string end = "") {
    //     string returned = recurrency_type.to_friendly_string (recurrency_interval);

    //     if (recurrency_type == RecurrencyType.EVERY_WEEK &&
    //         recurrency_weeks.split (",").length > 0) {
    //         string weeks = "";
    //         if (recurrency_weeks.contains ("1")) {
    //             weeks += _("Mo,");
    //         }

    //         if (recurrency_weeks.contains ("2")) {
    //             weeks += _("Tu,");
    //         }

    //         if (recurrency_weeks.contains ("3")) {
    //             weeks += _("We,");
    //         }

    //         if (recurrency_weeks.contains ("4")) {
    //             weeks += _("Th,");
    //         }

    //         if (recurrency_weeks.contains ("5")) {
    //             weeks += _("Fr,");
    //         }

    //         if (recurrency_weeks.contains ("6")) {
    //             weeks += _("Sa,");
    //         }

    //         if (recurrency_weeks.contains ("7")) {
    //             weeks += _("Su,");
    //         }

    //         weeks = weeks.slice (0, -1);
    //         returned = "%s (%s)".printf (returned, weeks);
    //     }

    //     return returned + " " + end;
    // }

    // pub static GLib.DateTime get_today_format_date () {
    //     return get_date_only (new DateTime.now_local ());
    // }

    // pub static GLib.DateTime get_date_only (GLib.DateTime date) {
    //     return new DateTime.local (
    //         date.get_year (),
    //         date.get_month (),
    //         date.get_day_of_month (),
    //         0,
    //         0,
    //         0
    //     );
    // }

    // pub static string get_default_date_format_from_date (GLib.DateTime? date) {
    //     if (date == null) {
    //         return "";
    //     }

    //     let format = date.format (Granite.DateTime.get_default_date_format (
    //         false,
    //         true,
    //         date.get_year () != new GLib.DateTime.now_local ().get_year ()
    //     ));
    //     return format;
    // }

    // pub static string get_todoist_datetime_format (GLib.DateTime date) {
    //     string returned = "";

    //     if (has_time (date)) {
    //         returned = date.format ("%F") + "T" + date.format ("%T");
    //     } else {
    //         returned = date.format ("%F");
    //     }

    //     return returned;
    // }

    // pub static bool has_time_from_string (string date) {
    //     return has_time (new GLib.DateTime.from_iso8601 (date, new GLib.TimeZone.local ()));
    // }

    // pub static int get_days_of_month (int index, int year_nav) {
    //     if ((index == 1) || (index == 3) || (index == 5) || (index == 7) || (index == 8) || (index == 10) || (index == 12)) { // vala-lint=line-length
    //         return 31;
    //     } else {
    //         if (index == 2) {
    //             if (year_nav % 4 == 0) {
    //                 return 29;
    //             } else {
    //                 return 28;
    //             }
    //         } else {
    //             return 30;
    //         }
    //     }
    // }

    // pub static GLib.DateTime get_start_of_month (owned GLib.DateTime? date = null) {
    //     if (date == null) {
    //         date = new GLib.DateTime.now_local ();
    //     }

    //     return new GLib.DateTime.local (date.get_year (), date.get_month (), 1, 0, 0, 0);
    // }

    // pub static bool is_current_month (GLib.DateTime date) {
    //     let now = new GLib.DateTime.now_local ();

    //     if (date.get_year () == now.get_year ()) {
    //         if (date.get_month () == now.get_month ()) {
    //             return true;
    //         } else {
    //             return false;
    //         }
    //     } else {
    //         return false;
    //     }
    // }

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
