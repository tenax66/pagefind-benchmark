use regex_benchmark::{UNICODE_REGEX, ASCII_REGEX, read_test_string_from_file};

fn main() {
    let file_path = "test_input.txt"; // 読み込むファイルパス
    match read_test_string_from_file(file_path) {
        Ok(test_string) => {
            println!(
                "Unicode Regex Match: {}",
                UNICODE_REGEX.is_match(&test_string)
            );
            println!(
                "ASCII Regex Match: {}",
                ASCII_REGEX.is_match(&test_string)
            );
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
        }
    }
}
