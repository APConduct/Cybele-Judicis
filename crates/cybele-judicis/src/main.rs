
mod audio;
mod ui;
mod project;
mod effects;
mod scripting;

use anyhow::Result;
use eframe::{egui, App, CreationContext};

fn main() -> Result<(), eframe::Error> {



    let options = eframe::NativeOptions::default();



    eframe::run_native(
        "Hello World",
        options,
        Box::new(|_cc| Ok(Box::new(HelloApp) as Box<dyn App>)),
    )
}

struct HelloApp;

impl App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, World!");
        });
    }
}