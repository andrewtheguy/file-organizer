use chrono::{DateTime, Utc};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let utc: DateTime<Utc> = Utc::now();
    println!("{}",utc.format("%Y%m%d%H%M%S%.3f").to_string());
    Ok(())
}