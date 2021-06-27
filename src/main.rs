#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app::App;
use eframe::{egui, NativeOptions};
mod app;
mod speaker;
mod utils;

const APP_NAME: &str = "Random Word Reader";
const APP_WIDTH: f32 = 320.;
const APP_HEIGHT: f32 = 340.;
const DEFAULT_INTERVAL: u64 = 30;

fn main() {
    let app = App::default();
    eframe::run_native(
        Box::new(app),
        NativeOptions {
            initial_window_size: Some(egui::Vec2 {
                x: APP_WIDTH,
                y: APP_HEIGHT,
            }),
            ..NativeOptions::default()
        },
    );
}
