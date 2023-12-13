use chord_map_egui::models::chord_context::ChordContext;
use eframe::egui::Id;
use note_lib::models::{Chord, ChordQuality, Note};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct ChordViewContext {
    id: Uuid,
    pub chord_context: ChordContext,
    pub window_open: bool,

    /// X offset from the center of the map
    pub map_x: f64,
    /// Y offset from the center of the map
    pub map_y: f64,
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

    pub fn set_position(&mut self, x: f64, y: f64) {
        self.map_x = x;
        self.map_y = y;
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
