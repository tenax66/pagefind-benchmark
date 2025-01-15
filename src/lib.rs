use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

lazy_static! {
    pub static ref UNICODE_REGEX: Regex = Regex::new(r"[^\w]").unwrap();
    pub static ref ASCII_REGEX: Regex = Regex::new(r"[^a-zA-Z_0-9]").unwrap();
}

/// テキストファイルから文字列を読み込む
pub fn read_test_string_from_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?; // ファイルを開く
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // ファイル内容を文字列に読み込む
    Ok(contents)
}
