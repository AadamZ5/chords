use egui::{ComboBox, DragValue, Ui, Vec2, Widget, WidgetText};
use note_lib::{Note, NoteModifier, RawNote};
use strum::IntoEnumIterator;

use crate::models::chord_context::ChordContext;

#[derive(Debug, Clone, PartialEq, Default)]
struct RawNoteOption {
    note: note_lib::RawNote,
    modifier: note_lib::NoteModifier,
}

impl RawNoteOption {
    fn new(note: note_lib::RawNote, modifier: note_lib::NoteModifier) -> Self {
        Self { note, modifier }
    }
}

impl Into<WidgetText> for RawNoteOption {
    fn into(self) -> WidgetText {
        WidgetText::RichText(
            format!("{}{}", self.note.to_string(), self.modifier.to_string()).into(),
        )
    }
}

impl Into<WidgetText> for &RawNoteOption {
    fn into(self) -> WidgetText {
        WidgetText::RichText(
            format!("{}{}", self.note.to_string(), self.modifier.to_string()).into(),
        )
    }
}
pub enum ChordEditAction {
    Commit,
    Cancel,
}

pub fn chord_edit(ui: &mut egui::Ui, chord_edit_ctx: &mut ChordContext) -> Option<ChordEditAction> {
    let current_root = chord_edit_ctx.get_root();
    let mut current_octave = current_root.octave();
    let mut current_root_and_modifier: RawNoteOption =
        RawNoteOption::new(current_root.raw_note(), current_root.modifier());

    let root_or_octave_changed = ui.horizontal(|ui| {
        let root_combo = ComboBox::new("Root", "")
            .width(50.0)
            .selected_text(&current_root_and_modifier)
            .show_ui(ui, |ui| {
                RawNote::iter()
                    .filter(|raw_note| match raw_note {
                        RawNote::Incongruent(_) => false,
                        _ => true,
                    })
                    .flat_map(|raw_note| {
                        [
                            RawNoteOption::new(raw_note, NoteModifier::Flat),
                            RawNoteOption::new(raw_note, NoteModifier::Natural),
                            RawNoteOption::new(raw_note, NoteModifier::Sharp),
                        ]
                    })
                    .map(|option| {
                        let widget_text: WidgetText = (&option).into();
                        let response = ui.selectable_value(
                            &mut current_root_and_modifier,
                            option,
                            widget_text,
                        );
                        response
                    })
                    .reduce(|a, b| a.union(b))
            });

        let octave_drag_box = DragValue::new(&mut current_octave)
            .speed(0.05)
            .clamp_range(0..=10)
            .ui(ui);

        let root_or_octacve_changed = root_combo
            .inner
            .flatten()
            .map(|r| r.clicked())
            .unwrap_or(false)
            || octave_drag_box.changed();

        root_or_octacve_changed
    });

    let commit = ui.allocate_ui(ui.available_size(), |ui| {
        ui.horizontal(|ui| {
            let commit_button = ui.button("✔");
            let cancel_button = ui.button("✖");

            if commit_button.clicked() {
                Some(ChordEditAction::Commit)
            } else if cancel_button.clicked() {
                Some(ChordEditAction::Cancel)
            } else {
                None
            }
        })
    });

    if root_or_octave_changed.inner {
        let RawNoteOption { note, modifier } = current_root_and_modifier;
        chord_edit_ctx.set_root(Note::new(note, current_octave, modifier))
    }

    commit.inner.inner
}
