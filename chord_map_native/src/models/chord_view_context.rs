use chord_map_egui::models::chord_context::ChordContext;
use eframe::{egui::Id, epaint::Pos2};
use note_lib::{Chord, ChordQuality, Note};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct ChordViewContext {
    id: Uuid,
    pub chord_context: ChordContext,
    pub editing_chord_context: Option<ChordContext>,
    pub window_open: bool,

    pub map_pos: Pos2,
}

impl ChordViewContext {
    pub fn new(chord_root: Note, chord_quality: ChordQuality) -> Self {
        Self {
            id: Uuid::new_v4(),
            window_open: false,
            chord_context: ChordContext::new(chord_root, chord_quality),

            ..Default::default()
        }
    }

    pub fn set_position(&mut self, pos: Pos2) {
        self.map_pos = pos;
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
}

impl Into<Id> for ChordViewContext {
    fn into(self) -> Id {
        Id::new(self.id.to_string())
    }
}

impl Into<Id> for &ChordViewContext {
    fn into(self) -> Id {
        Id::new(self.id.to_string())
    }
}
