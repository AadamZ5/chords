use std::fmt::format;

use egui::{Grid, Response, Ui, Widget};
use note_lib::models::Chord;

use crate::models::chord_context::ChordContext;

pub fn chord_view(ui: &mut Ui, chord_ctx: &mut ChordContext) {
    let label_response = ui.heading(format!("{}", chord_ctx));

    let note_grid_response = Grid::new("note_grid")
        .spacing([2.0, 2.0])
        .min_col_width(0.0)
        .show(ui, |ui| {
            chord_ctx
                .get_calculated_chord()
                .notes()
                .iter()
                .map(|note| ui.small_button(format!("{:#}", note)))
                .reduce(|a, b| a.union(b))
        });
}
