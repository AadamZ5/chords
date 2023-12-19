use crate::{Interval, Note, NoteModifier};

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

impl From<Interval> for Semitone {
    fn from(value: Interval) -> Self {
        value.semitones()
    }
}

impl From<Note> for Semitone {
    fn from(value: Note) -> Self {
        value.to_semitones_from_c0() as i32
    }
}
