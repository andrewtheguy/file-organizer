use std::fs;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("Cargo.lock")?;

    if let Ok(time) = metadata.created() {
        println!("{time:?}");
    } else {
        println!("Not supported on this platform or filesystem");
    }
    Ok(())
}