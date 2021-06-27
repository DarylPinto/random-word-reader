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
        .unwrap()
        .map(|entry| entry.unwrap().path())
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
            .map(|entry| entry.unwrap().path())
            .collect(),
    };

    let mut all_words: Vec<String> = vec![];

    for path in paths {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        let mut words: Vec<String> = buffer
            .lines()
            .map(|line| line.expect("unable to parse"))
            .collect();

        all_words.append(&mut words);
    }

    Ok(all_words)
}

pub fn path_to_string(path: &Path) -> String {
    path.file_stem()
        .unwrap()
        .to_owned()
        .into_string()
        .unwrap_or_else(|_| String::from("Unknown"))
}
