use rand::seq::SliceRandom;
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
    sync::mpsc::Receiver,
};
use tts::Tts;

pub fn get_word_filenames() -> Vec<(PathBuf, String)> {
    std::fs::read_dir("./words")
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let stem = path.file_stem().unwrap().to_owned().into_string().unwrap();
            (path, stem)
        })
        .collect()
}

pub fn speak(
    interval: u64,
    categories: Vec<(PathBuf, String)>,
    selected_category: String,
    rx: Receiver<bool>,
) -> Result<(), Box<dyn Error>> {
    // Get filename(s) for selected word type
    let paths: Vec<PathBuf> = if &selected_category == "All" {
        std::fs::read_dir("./words")?
            .map(|entry| entry.unwrap().path())
            .collect()
    } else {
        let path = &categories
            .iter()
            .find(|t| t.1 == selected_category)
            .unwrap()
            .0;
        vec![path.clone()]
    };

    let mut all_words: Vec<String> = vec![];

    // Load words into memory
    for path in paths {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        let mut words: Vec<String> = buffer
            .lines()
            .map(|line| line.expect("unable to parse"))
            .collect();

        all_words.append(&mut words);
    }

    // Speak
    let mut speaker = Tts::default()?;
    let delay = if interval < 1 { 1 } else { interval };

    loop {
        let choice = all_words.choose(&mut rand::thread_rng());
        if let Some(word) = choice {
            speaker.speak(word, true)?;
        }
        if let Ok(should_stop) = rx.try_recv() {
            if should_stop {
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(delay));
    }

    Ok(())
}
