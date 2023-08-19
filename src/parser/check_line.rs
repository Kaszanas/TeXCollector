use crate::parser::find_brackets::find_brackets;

// REVIEW: This looks bad, is there a better way to do this?
/// Checks a line in search for a specific command substring.
///
/// # Example
///
/// ```
/// use texcollector::parser::check_line::check_line;
///
/// let line = "\\command{with_enclosed_content}".to_string();
///
/// if let Some(found_command_content) = check_line(line, "\\command") {
///     println!("Got command content: {}", found_command_content);
/// }
/// ```
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
