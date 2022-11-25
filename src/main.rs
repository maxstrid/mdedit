use eframe::egui;

mod gui;
mod markdown;

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Markdown Editor",
        options,
        Box::new(|_| Box::new(gui::Gui::default())),
    )
}
