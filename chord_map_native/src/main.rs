mod models;
mod ui;

use std::{
    fmt::format,
    sync::{
        Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError, TryLockResult,
    },
};

use eframe::egui::Window;
use models::chord_map_state::ChordMapState;
use models::chord_view_context::ChordViewContext;
use note_lib::models::{Chord, Note, RawNote};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Chord Map",
        options,
        Box::new(|_cc| Box::<ChordMapApp>::default()),
    )
}

#[derive(Debug, Default)]
struct ChordMapApp {
    chord_map_state: ChordMapState,
}

impl ChordMapApp {
    fn new() -> Self {
        Self::default()
    }
}

impl eframe::App for ChordMapApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ui::static_ui::main_ui(ctx, &mut self.chord_map_state);
        ui::static_ui::chords_edit_windows(ctx, &mut self.chord_map_state);
        ui::static_ui::chords_display(ctx, &mut self.chord_map_state);
    }
}
