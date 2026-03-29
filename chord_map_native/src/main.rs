mod models;
mod ui;

use models::chord_map_state::ChordMapState;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Chord Map",
        options,
        Box::new(|_cc| Ok(Box::<ChordMapApp>::default())),
    )
}

#[derive(Debug, Default)]
struct ChordMapApp {
    chord_map_state: ChordMapState,
}

impl eframe::App for ChordMapApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        ui::static_ui::main_ui(ui, &mut self.chord_map_state);
    }
}
