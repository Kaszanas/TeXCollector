// REVIEW: Should this be more universal?
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
