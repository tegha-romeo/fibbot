use std::env;

fn main() {
    let enable_fibb: bool = env::var("INPUT_ENABLE_FIBB")
        .unwrap_or("true".to_string())
        .parse()
        .unwrap_or(true);

    let max_threshold: u32 = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or("100".to_string())
        .parse()
        .unwrap_or(100);

    println!("enable_fibb: {}", enable_fibb);
    println!("Max threshold is {}", max_threshold);
}