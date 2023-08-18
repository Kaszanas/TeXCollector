use crate::parser::find_brackets::find_brackets;

// REVIEW: This looks bad, is there a better way to do this?
pub fn check_line(line: String, command: &str) -> Option<String> {
    match line.contains(command) {
        true => {
            log::info!("line contains command");
            log::info!("Full line: {}", line);
            find_brackets(line)
        }
        false => return None,
    }
}
