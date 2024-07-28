fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: i32, where_from: String, message: String) {
    println!("[line {} ] Error {}: {}", line, where_from, message);
    let _had_error = true;
}
