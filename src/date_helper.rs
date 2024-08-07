use chrono::NaiveDate;
pub fn parse_date(date_str: &str) -> Result<NaiveDate, &str> {
    for p in date_str.split(' ').into_iter() {
        if let Ok(date) = NaiveDate::parse_from_str(p, "%Y/%m/%d") {
            return Ok(date);
        }
        if let Ok(date) = NaiveDate::parse_from_str(p, "%m/%d/%Y") {
            return Ok(date);
        }
    }
    Err("parse failed")
}