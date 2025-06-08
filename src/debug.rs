pub fn info(message: &str) {
    println!("[{}] [info] {message}", now())
}

fn now() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
