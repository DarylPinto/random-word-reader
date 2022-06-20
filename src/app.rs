use crate::speaker;
use crate::utils::{self, ChannelMessage};
use crate::APP_NAME;
use crate::DEFAULT_INTERVAL;
use eframe::{egui, epi};
use std::{path::PathBuf, sync::mpsc};

const PADDING: f32 = 10.;

pub struct App {
    interval: u64,
    categories: Vec<PathBuf>,
    selected_category: Option<PathBuf>,
    is_speaking: bool,
    channel_transmitter: Option<mpsc::Sender<ChannelMessage>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            interval: DEFAULT_INTERVAL,
            categories: utils::get_word_filenames(),
            selected_category: None,
            is_speaking: false,
            channel_transmitter: None,
        }
    }
}

// App header
fn render_header(ui: &mut egui::Ui) {
    ui.heading(APP_NAME);
    ui.add_space(PADDING);
}

// Start screen
fn render_main_screen(app: &mut App, ui: &mut egui::Ui) {
    ui.radio_value(&mut app.selected_category, None, "All");
    for category in &app.categories {
        ui.radio_value(
            &mut app.selected_category,
            Some(category.clone()),
            utils::path_to_string(category),
        );
    }
    ui.add_space(PADDING);
    ui.add(egui::Slider::new(&mut app.interval, 1..=120).text("delay between words"));
    ui.add_space(PADDING * 2.);
    if ui.button("   Start Reading   ").clicked() {
        app.is_speaking = true;
        let categories = app.categories.clone();
        let selected = app.selected_category.clone();
        let interval = app.interval;
        let (tx, rx) = mpsc::channel();
        app.channel_transmitter = Some(tx);
        std::thread::spawn(move || {
            speaker::speak(categories, selected, interval, rx).unwrap_or(());
        });
    }
}

// Speaker Screen
fn render_speaking_screen(app: &mut App, ui: &mut egui::Ui) {
    let selected_label = match &app.selected_category {
        Some(cat) => utils::path_to_string(&cat),
        None => String::from("All"),
    };

    ui.add_space(PADDING);
    ui.label("Keep your ears open :)");
    ui.add_space(PADDING);
    ui.label(format!(
        "Listening to: {} words, {} second(s) apart",
        selected_label, app.interval
    ));
    ui.add_space(PADDING * 2.);
    if ui.button("Back").clicked() {
        if let Some(transmitter) = &app.channel_transmitter {
            if transmitter.send(ChannelMessage::StopTalking).is_ok() {
                app.is_speaking = false;
            }
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        APP_NAME
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            render_header(ui);
            if !self.is_speaking {
                render_main_screen(self, ui);
            } else {
                render_speaking_screen(self, ui);
            }
        });
    }
}
