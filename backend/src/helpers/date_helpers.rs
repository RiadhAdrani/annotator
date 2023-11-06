use chrono::{prelude::*, Duration};

pub fn format_datetime(datetime: DateTime<Local>) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn parse_datetime_from_string(
    datetime_string: &str,
) -> Result<DateTime<Local>, chrono::format::ParseError> {
    let parsed_datetime = DateTime::parse_from_str(datetime_string, "%Y-%m-%d %H:%M:%S");

    if parsed_datetime.is_err() {
        return Err(parsed_datetime.err().unwrap());
    }

    let local_datetime = parsed_datetime.unwrap().with_timezone(&Local);

    Ok(local_datetime)
}

pub fn create_datetime_with_days_offset(days_offset: i64) -> DateTime<Local> {
    // Get the current time
    let current_time = Local::now();

    // Create a new DateTime with a 7-day offset from the current date
    let future_time = current_time + Duration::days(days_offset);

    future_time
}

pub fn is_datetime_in_past(datetime: &DateTime<Local>) -> bool {
    let current_time = Local::now();

    datetime < &current_time
}
