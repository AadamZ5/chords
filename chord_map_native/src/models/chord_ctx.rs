use eframe::egui::Id;
use note_lib::models::Chord;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct ChordCtx {
    id: Uuid,
    pub chord: Chord,
    pub window_open: bool,

    /// X offset from the center of the map
    pub map_x: f64,
    /// Y offset from the center of the map
    pub map_y: f64,
}

impl ChordCtx {
    pub fn new(chord: Chord) -> Self {
        Self {
            id: Uuid::new_v4(),
            window_open: false,
            chord,

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

impl Into<Id> for ChordCtx {
    fn into(self) -> Id {
        Id::new(self.id.to_string())
    }
}

impl Into<Id> for &ChordCtx {
    fn into(self) -> Id {
        Id::new(self.id.to_string())
    }
}
