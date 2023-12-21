use super::{chord_quality::ChordQuality, Chord};
use crate::Note;

pub struct ChordBuilder {
    root: Note,
    quality: Option<ChordQuality>,
    additions: Vec<Note>,
}

impl ChordBuilder {
    pub fn new(root: Note) -> Self {
        ChordBuilder {
            root,
            quality: None,
            additions: Vec::new(),
        }
    }

    pub fn quality(mut self, quality: ChordQuality) -> Self {
        self.quality = Some(quality);
        self
    }

    pub fn add_note(mut self, note: Note) -> Self {
        self.additions.push(note);
        self
    }

    pub fn build(self) -> Chord {
        let mut notes = vec![self.root];
        if let Some(quality) = self.quality {
            notes.extend(quality.to_notes(self.root));
        }
        notes.extend(self.additions);
        Chord::new(notes)
    }
}
