// REVIEW: This does not take into account unbalanced parentheses.
// REVIEW: But it may not be required, we assume that
// REVIEW: LaTeX has matching parentheses always

// REVIEW: Should this be more universal?
/// Finds opening and closing curly brackets, and returns the content
/// that was enclosed.
///
/// # Examples
///
/// ```
/// use texcollector::parser::find_brackets::find_brackets;
///
/// let content = "this is a {get_this_enclosed_content} test".to_string();
/// if let Some(found_content) = find_brackets(content) {
///     println!("Got: {}", found_content)
/// }
/// ```
pub fn find_brackets(line: &str) -> Option<&str> {
    let open = line.find("{")?;

    // Reversing the line and finding first parentheses:
    let line_len = line.len();
    let reversed_line: String = line.chars().rev().collect();

    // Getting the index from the right side for the closing parentheses:
    let close = line_len - reversed_line.find("}")?;
    let extracted_text = &line[open + 1..close];

    Some(extracted_text)
}
