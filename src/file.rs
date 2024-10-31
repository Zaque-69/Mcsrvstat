pub fn format_words(words: &[&str]) -> String {
    words.join("\n")
}

use std::fs::{ OpenOptions, File };
use std::io::{ Write, Result };
pub fn write_server_info(file_path: &str, text: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;

    // Scrie textul în fișier
    writeln!(file, "{}", text)?;

    Ok(())
}

use std::fs;
pub fn clear_server_data() -> Result<()>  {
    // Removing all the content from the file with the info
    fs::remove_file("serverinfo").ok();
    File::create("serverinfo")?;

    Ok(())
}

use std::path::Path;
use crate::io::BufReader;
use std::io::BufRead;
pub fn read_words_from_file(filename: &str) -> Vec<String> {
    // 
    let path = Path::new(filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return vec![],  
    };

    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect()
}

const SPACE: &str = " ";
const TWO_SPACES: &str = "  ";
pub fn trim_whitespace(s: &str) -> String {
    let mut new_str: String = s.trim().to_owned();
    while new_str.contains(TWO_SPACES) {
        new_str = new_str.replace(TWO_SPACES, SPACE);
    }
    new_str
}
