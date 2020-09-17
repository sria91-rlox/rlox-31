pub fn error(line: u32, message: &str) {
  report_error(line, "", message);
}

fn report_error(line: u32, location: &str, message: &str) {
  println!("[line {}] Error {}: {}", line, location, message);
}
