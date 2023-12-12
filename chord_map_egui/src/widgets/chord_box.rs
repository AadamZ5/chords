use egui::Widget;
use note_lib::models::Chord;

pub fn chord_summary_widget(ui: &mut egui::Ui, chord: &mut Chord) -> egui::Response {
    let inner = ui.group(|ui| {
        ui.label("Chord: ");

        for note in chord.notes() {
            ui.label(format!("\t{:#}", note));
        }

        //ui.separator();
        let button = ui.button("Add Note");

        if button.clicked() {
            chord.add_note(note_lib::models::Note::new(note_lib::models::RawNote::A, 4));
        }
    });

    inner.response
}

pub fn chord_summary(chord: &mut Chord) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| chord_summary_widget(ui, chord)
}

#[derive(Debug)]
pub struct ChordModifier<'a> {
    chord: &'a mut Chord,
}

impl<'a> ChordModifier<'a> {
    pub fn new(chord: &'a mut Chord) -> Self {
        Self { chord }
    }

    pub fn add_note(&mut self, note: note_lib::models::Note) {
        self.chord.add_note(note);
    }

    pub fn get_output_chord(&self) -> &Chord {
        &self.chord
    }
}

impl<'a> Widget for ChordModifier<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        chord_summary_widget(ui, self.chord)
    }
}
