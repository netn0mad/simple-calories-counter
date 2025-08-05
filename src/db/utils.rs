use time::{format_description, OffsetDateTime};

pub fn get_formated_datetime(datetime: OffsetDateTime) -> String {
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ).unwrap();

    datetime.format(&format).unwrap()
}