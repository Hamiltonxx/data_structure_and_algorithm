// Standard library only provides std::time::Instant for measuring elapsed time and
// std::time::Duration for durations
// Use chrono crate to handle date and time

// cargo test -- --nocapture
#[test]
fn test_datetime() {
    use chrono::{DateTime, Utc, Local};
    
    // DateTime<Utc>, DateTime<Local>, DateTime<Tz>
    let utc_now: DateTime<Utc> = Utc::now();
    println!("Current UTC time: {}", utc_now);
    let local_now: DateTime<Local> = Local::now();
    println!("Current local time: {}", local_now);

    // Format a date and time 
    let formatted_time = local_now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted local time: {}", formatted_time);

    // Parse a string into a DateTime 
    let parsed_time = DateTime::parse_from_rfc3339("2024-11-12T12:00:00+00:00").expect("Failed to parse time");
    println!("Parsed time: {}", parsed_time);
}

#[test]
fn test_naivedatetime() {
    use chrono::{NaiveDate, NaiveDateTime, Duration}; // without timezone info 
    let date = NaiveDate::from_ymd(2024, 11, 12);
    let time = date.and_hms(12, 0, 0);
    println!("NaiveDateTime: {}", time);
    let timestamp = 1_000_000_000;
    let naive_date_time = NaiveDateTime::from_timestamp(timestamp, 0);
    println!("NaiveDateTime from timestamp: {}", naive_date_time);
    let formatted = naive_date_time.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted NaiveDateTime: {}", formatted);
    // Add 10 days, Subtract 5 hours
    let new_date_time = naive_date_time + Duration::days(10);
    println!("After adding 10 days: {}", new_date_time);
    let earlier_time = new_date_time - Duration::hours(5);
    println!("5 hours earlier: {}", earlier_time);
}
