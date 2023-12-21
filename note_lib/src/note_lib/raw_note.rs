use std::{fmt::Display, ops::Add};

use strum_macros::EnumIter;

use super::{AbstractNote, NoteModifier};
use crate::{Hertz, Semitone};

#[derive(PartialEq, Clone, Copy, Debug, Default, EnumIter)]
pub enum RawNote {
    /// A note that does not fit on the largely used 12-tone scale.
    Incongruent(Hertz),
    #[default]
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl RawNote {
    pub fn next_note(&self) -> (RawNote, Semitone) {
        match self {
            RawNote::C => (RawNote::D, 2),
            RawNote::D => (RawNote::E, 2),
            RawNote::E => (RawNote::F, 1),
            RawNote::F => (RawNote::G, 2),
            RawNote::G => (RawNote::A, 2),
            RawNote::A => (RawNote::B, 2),
            RawNote::B => (RawNote::C, 1),
            RawNote::Incongruent(_) => panic!(),
        }
    }

    pub fn prev_note(&self) -> (RawNote, Semitone) {
        match self {
            RawNote::C => (RawNote::B, 1),
            RawNote::D => (RawNote::C, 2),
            RawNote::E => (RawNote::D, 2),
            RawNote::F => (RawNote::E, 1),
            RawNote::G => (RawNote::F, 2),
            RawNote::A => (RawNote::G, 2),
            RawNote::B => (RawNote::A, 2),
            RawNote::Incongruent(_) => panic!(),
        }
    }

    pub fn raw_note_to_hz(raw_note: RawNote) -> Hertz {
        // I referenced https://pages.mtu.edu/~suits/notefreqs.html for the frequencies.

        // TODO: Implement the MIDI algorithm from https://newt.phys.unsw.edu.au/jw/notes.html instead!
        match raw_note {
            RawNote::Incongruent(hz) => hz,
            RawNote::C => 16.35,
            RawNote::D => 18.35,
            RawNote::E => 20.60,
            RawNote::F => 21.83,
            RawNote::G => 24.50,
            RawNote::A => 27.50,
            RawNote::B => 30.87,
        }
    }

    pub fn to_hertz(&self) -> Hertz {
        RawNote::raw_note_to_hz(*self)
    }
}

pub const C: RawNote = RawNote::C;
pub const D: RawNote = RawNote::D;
pub const E: RawNote = RawNote::E;
pub const F: RawNote = RawNote::F;
pub const G: RawNote = RawNote::G;
pub const A: RawNote = RawNote::A;
pub const B: RawNote = RawNote::B;

impl Display for RawNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let note = match self {
            RawNote::Incongruent(_) => "Incongruent",
            RawNote::C => "C",
            RawNote::D => "D",
            RawNote::E => "E",
            RawNote::F => "F",
            RawNote::G => "G",
            RawNote::A => "A",
            RawNote::B => "B",
        };

        write!(f, "{}", note)
    }
}

pub enum IntoRawNoteError {
    InvalidNoteChar(char),
}

impl TryFrom<char> for RawNote {
    type Error = IntoRawNoteError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !value.is_alphabetic() {
            return Err(IntoRawNoteError::InvalidNoteChar(value));
        }

        match value.to_ascii_uppercase() {
            'C' => Ok(RawNote::C),
            'D' => Ok(RawNote::D),
            'E' => Ok(RawNote::E),
            'F' => Ok(RawNote::F),
            'G' => Ok(RawNote::G),
            'A' => Ok(RawNote::A),
            'B' => Ok(RawNote::B),
            _ => Err(IntoRawNoteError::InvalidNoteChar(value)),
        }
    }
}

impl Add<NoteModifier> for RawNote {
    type Output = AbstractNote;

    fn add(self, rhs: NoteModifier) -> Self::Output {
        AbstractNote {
            modifier: rhs,
            raw_note: self,
        }
    }
}
