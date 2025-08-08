use time::{format_description, OffsetDateTime};

pub fn get_formated_datetime(datetime: OffsetDateTime) -> String {
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ).unwrap();

    datetime.format(&format).unwrap()
}

pub fn get_formated_datetime_to_day(datetime: OffsetDateTime) -> String {
    let format = format_description::parse(
        "[year]-[month]-[day]"
    ).unwrap();

    datetime.format(&format).unwrap()
}

pub fn get_current_datetime() -> OffsetDateTime {
    match OffsetDateTime::now_local() {
        Ok(local_time) => local_time,
        Err(_) => OffsetDateTime::now_utc(),
    }
}