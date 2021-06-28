use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
};

pub enum ChannelMessage {
    StopTalking,
}

// Get list of filenames for word categories
pub fn get_word_filenames() -> Vec<PathBuf> {
    std::fs::read_dir("./words")
        .expect("Unable to access word category folder")
        .map(|entry| entry.expect("Failed to get path for entry").path())
        .collect()
}

// Get list of words to read from based on the selected category
pub fn get_words(
    categories: Vec<PathBuf>,
    selected_category: Option<PathBuf>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let paths: Vec<PathBuf> = match &selected_category {
        Some(selected) => {
            let path = categories.iter().find(|&t| t == selected).unwrap();
            vec![path.clone()]
        }
        None => std::fs::read_dir("./words")?
            .map(|entry| entry.expect("Failed to get path for entry").path())
            .collect(),
    };

    let mut all_words: Vec<String> = vec![];

    for path in paths {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        let mut words: Vec<String> = buffer
            .lines()
            .map(|line| line.expect("Unable to parse line"))
            .collect();

        all_words.append(&mut words);
    }

    Ok(all_words)
}

pub fn path_to_string(path: &Path) -> String {
    path.file_stem()
        .expect(&format!("No filename for {:?}!", path))
        .to_owned()
        .into_string()
        .unwrap_or_else(|_| String::from("Unknown"))
}
