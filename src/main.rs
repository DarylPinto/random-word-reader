// #![windows_subsystem = "windows"]
use eframe::{egui, epi};
use rand::seq::SliceRandom;
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use tts::Tts;

const APP_NAME: &str = "Random Word Reader";
const DEFAULT_INTERVAL: u64 = 30;
const PADDING: f32 = 10.0;

#[derive(Clone, Copy, PartialEq, EnumIter, Display)]
enum WordType {
    All,
    Noun,
    Verb,
    Adjective,
    Question,
    Idiom,
    Vocabulary,
    Custom
}

pub struct App {
    interval: u64,
    word_type: WordType,
    is_speaking: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            interval: DEFAULT_INTERVAL,
            word_type: WordType::All,
            is_speaking: false,
        }
    }
}

fn speak(interval: u64, word_type: WordType) -> Result<(), Box<dyn Error>> {
    // Get filename(s) for selected word type
    let word_filenames: Vec<_> = match word_type {
        WordType::All => std::fs::read_dir("./words")?
            .map(|entry| entry.unwrap().path())
            .collect(),
        _ => {
            let filename = format!("./words/{}.txt", word_type.to_string().to_lowercase());
            let path = std::path::PathBuf::from(filename);
            vec![path]
        }
    };

    let mut all_words: Vec<String> = vec![];

    // Load words into memory
    for filename in word_filenames {
        let file = File::open(filename)?;
        let buffer = BufReader::new(file);

        let mut words: Vec<String> = buffer
            .lines()
            .map(|line| line.expect("unable to parse"))
            .collect();

        all_words.append(&mut words);
    }

    // Speak
    let mut speaker = Tts::default()?;

    loop {
        match all_words.choose(&mut rand::thread_rng()) {
            Some(w) => {
                speaker.speak(w, true).unwrap();
            }
            None => (),
        }
        let delay = if interval < 1 { 1 } else { interval };
        std::thread::sleep(std::time::Duration::from_secs(delay));
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        APP_NAME
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.heading(APP_NAME);
            ui.add_space(PADDING);
            if !self.is_speaking {
                for category in WordType::iter() {
                    ui.radio_value(&mut self.word_type, category, category.to_string());
                }
                ui.add_space(PADDING);
                ui.add(egui::Slider::new(&mut self.interval, 1..=120).text("delay between words"));
                ui.add_space(PADDING * 2.);
                if ui.button("   Start Reading   ").clicked() {
                    self.is_speaking = true;
                    let interval = self.interval.clone();
                    let word_type = self.word_type.clone();
                    std::thread::spawn(move || {
                        speak(interval, word_type);
                    });
                }
            } else {
                ui.add_space(PADDING);
                ui.label("Keep your ears open :)");
                ui.add_space(PADDING);
                ui.label(format!(
                    "Listening to: {} words, {} second(s) apart",
                    self.word_type.to_string(),
                    self.interval
                ));
            }
        });
    }
}

fn main() {
    let app = App::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(app),
        eframe::NativeOptions {
            initial_window_size: Some(egui::Vec2 { x: 320., y: 340. }),
            resizable: false,
            ..options
        },
    );
}
