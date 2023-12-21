use std::fmt::{Display, Formatter};

use note_lib::{Chord, ChordQuality, Note};

#[derive(Debug, Default, Clone)]
pub struct ChordContext {
    root: Note,
    quality: ChordQuality,

    calculated_chord: Option<Chord>,
}

impl ChordContext {
    pub fn new(root: Note, quality: ChordQuality) -> Self {
        Self {
            root,
            quality,
            calculated_chord: None,
        }
    }

    pub fn get_root(&self) -> Note {
        self.root
    }

    pub fn get_quality(&self) -> ChordQuality {
        self.quality
    }

    pub fn set_root(&mut self, root: Note) {
        self.root = root;
        self.calculated_chord = None;
    }

    pub fn set_quality(&mut self, quality: ChordQuality) {
        self.quality = quality;
        self.calculated_chord = None;
    }

    pub fn get_calculated_chord(&mut self) -> &Chord {
        if self.calculated_chord.is_some() {
            self.calculated_chord.as_ref().unwrap()
        } else {
            let chord = self._calculate_chord();
            self.calculated_chord.replace(chord);
            self.calculated_chord.as_ref().unwrap()
        }
    }

    fn _calculate_chord(&self) -> Chord {
        self.quality.to_chord(self.root)
    }
}

impl Display for ChordContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#} {}", self.root, self.quality.short_name())
    }
}
