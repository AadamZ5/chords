use std::fmt::format;

use egui::{Response, Widget};
use note_lib::models::Chord;

use crate::models::chord_context::ChordContext;

#[derive(Debug)]
pub struct ChordView<'a> {
    chord_ctx: &'a mut ChordContext,
}

impl<'a> ChordView<'a> {
    pub fn new(chord: &'a mut ChordContext) -> Self {
        Self { chord_ctx: chord }
    }
}

impl<'a> Widget for ChordView<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let chord_ctx = self.chord_ctx;

        let label_response = ui.heading(format!("{}", chord_ctx));

        let combined_note_responses = chord_ctx
            .get_calculated_chord()
            .notes()
            .iter()
            .map(|note| ui.label(format!("\t{:#}", note)))
            .reduce(|a, b| a.union(b));

        if let Some(combined_note_responses) = combined_note_responses {
            combined_note_responses.union(label_response)
        } else {
            label_response
        }
    }
}
