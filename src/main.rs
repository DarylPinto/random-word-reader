#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::{egui, epi};
use std::path::PathBuf;
mod utils;

const APP_NAME: &str = "Random Word Reader";
const DEFAULT_INTERVAL: u64 = 30;
const PADDING: f32 = 10.0;

pub struct App {
    interval: u64,
    categories: Vec<(PathBuf, String)>,
    selected_category: String,
    is_speaking: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            interval: DEFAULT_INTERVAL,
            categories: utils::get_word_filenames(),
            selected_category: String::from("All"),
            is_speaking: false,
        }
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
                ui.radio_value(&mut self.selected_category, "All".to_string(), "All");
                for category in &self.categories {
                    ui.radio_value(
                        &mut self.selected_category,
                        category.1.to_string(),
                        category.1.to_string(),
                    );
                }
                ui.add_space(PADDING);
                ui.add(egui::Slider::new(&mut self.interval, 1..=120).text("delay between words"));
                ui.add_space(PADDING * 2.);
                if ui.button("   Start Reading   ").clicked() {
                    self.is_speaking = true;
                    let interval = self.interval;
                    let selected = self.selected_category.clone();
                    let categories = self.categories.clone();
                    std::thread::spawn(move || {
                        utils::speak(interval, categories, selected).unwrap_or(());
                    });
                }
            } else {
                ui.add_space(PADDING);
                ui.label("Keep your ears open :)");
                ui.add_space(PADDING);
                ui.label(format!(
                    "Listening to: {} words, {} second(s) apart",
                    self.selected_category.to_string(),
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
            ..options
        },
    );
}
