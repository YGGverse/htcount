pub fn info(message: &str) {
    println!("[{}] [info] {message}", now())
}

fn now() -> String {
    chrono::Local::now().to_rfc3339()
}
