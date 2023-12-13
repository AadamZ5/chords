use egui::{ComboBox, Widget, WidgetText};
use note_lib::models::{Chord, Note, RawNote};
use strum::IntoEnumIterator;

use crate::models::chord_context::ChordContext;

#[derive(Debug)]
pub struct ChordEdit<'a> {
    chord: &'a mut ChordContext,
}

impl<'a> ChordEdit<'a> {
    pub fn new(chord: &'a mut ChordContext) -> Self {
        Self { chord }
    }

    pub fn add_note(&mut self, note: note_lib::models::Note) {
        todo!();
        //self.chord.add_note(note);
    }

    pub fn get_output_chord(&mut self) -> &Chord {
        self.chord.get_calculated_chord()
    }
}

struct RawNoteWidgetText<'a> {
    note: &'a note_lib::models::RawNote,
}

impl<'a> From<&'a RawNote> for RawNoteWidgetText<'a> {
    fn from(note: &'a RawNote) -> Self {
        Self { note }
    }
}

impl Into<WidgetText> for RawNoteWidgetText<'_> {
    fn into(self) -> WidgetText {
        WidgetText::RichText(format!("{:#}", self.note.to_basic_notation()).into())
    }
}

impl<'a> Widget for ChordEdit<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let chord = self.chord;

        let mut current_root = chord.get_root();
        let mut current_raw_note = current_root.raw_note();

        let root = ComboBox::from_label("Root").show_ui(ui, |ui| {
            RawNote::iter()
                .filter(|raw_note| match raw_note {
                    RawNote::Incongruent(_) => false,
                    _ => true,
                })
                .map(|raw_note| {
                    let widget_text: WidgetText = RawNoteWidgetText::from(&raw_note).into();
                    let response =
                        ui.selectable_value(&mut current_raw_note, raw_note, widget_text);
                    response
                })
                .reduce(|a, b| a.union(b))
        });

        if let Some(inner) = root.inner.flatten() {
            if inner.clicked() {
                chord.set_root(Note::new(current_raw_note, 4))
            }
        }

        let inner = ui.group(|ui| {
            for note in chord.get_calculated_chord().notes() {
                ui.label(format!("\t{:#}", note));
            }

            //ui.separator();
            let button = ui.button("Add Note");

            if button.clicked() {
                //chord.add_note(note_lib::models::Note::new(note_lib::models::RawNote::A, 4));
            }
        });

        inner.response
    }
}
