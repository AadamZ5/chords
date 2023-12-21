use crate::{Note, NoteModifier, SimpleInterval};

pub type Semitone = i32;

impl From<NoteModifier> for Semitone {
    fn from(value: NoteModifier) -> Self {
        match value {
            NoteModifier::Sharp => 1,
            NoteModifier::Flat => -1,
            NoteModifier::Natural => 0,
            NoteModifier::DoubleSharp => 2,
            NoteModifier::DoubleFlat => -2,
        }
    }
}

impl From<SimpleInterval> for Semitone {
    fn from(value: SimpleInterval) -> Self {
        value.semitones()
    }
}

impl From<&SimpleInterval> for Semitone {
    fn from(value: &SimpleInterval) -> Self {
        value.semitones()
    }
}

impl From<Note> for Semitone {
    fn from(value: Note) -> Self {
        value.to_semitones_from_c0()
    }
}
