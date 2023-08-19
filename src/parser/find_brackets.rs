// REVIEW: Should this be more universal?
/// Finds opening and closing curly brackets, and returns the content
/// that was enclosed.
///
/// # Examples
///
/// ```
/// let content = "this is a {get_this_enclosed_content} test".to_string()
/// if let Some(find_brackets(content)) = found_content {
///     println!("Got: {}", found_content)
/// }
/// ```
pub fn find_brackets(line: String) -> Option<String> {
    match line.find("{") {
        Some(open) => match line.find("}") {
            Some(close) => {
                let extracted_text = &line[open + 1..close];
                log::info!("Extracted text from line: {}", extracted_text);
                return Some(extracted_text.to_owned());
            }
            _ => return None,
        },
        _ => return None,
    }
}
