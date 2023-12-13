use std::{
    default,
    fmt::{Display, Formatter},
    ops::Add,
    vec,
};
use strum_macros::EnumIter;

use crate::types::Hertz;

use super::Chord;

static STRING_INPUTS_TO_RAW_NOTE_MAP: [(RawNote, [&str; 0]); 0] = [];

#[derive(PartialEq, Clone, Copy, Debug, Default, EnumIter)]
pub enum RawNote {
    /// A note that does not fit on the largely used 12-tone scale.
    Incongruent(Hertz),
    CFlat,
    #[default]
    C,
    CSharp,
    DFlat,
    D,
    DSharp,
    EFlat,
    E,
    ESharp,
    FFlat,
    F,
    FSharp,
    GFlat,
    G,
    GSharp,
    AFlat,
    A,
    ASharp,
    BFlat,
    B,
    BSharp,
}

impl RawNote {
    pub fn is_analogous_to(&self, rhs: &RawNote) -> bool {
        if self == rhs {
            return true;
        }

        let rhs = *rhs;

        match self {
            RawNote::CFlat => rhs == RawNote::B,
            RawNote::B => rhs == RawNote::CFlat,

            RawNote::DFlat => rhs == RawNote::CSharp,
            RawNote::CSharp => rhs == RawNote::DFlat,

            RawNote::EFlat => rhs == RawNote::DSharp,
            RawNote::DSharp => rhs == RawNote::EFlat,

            RawNote::FFlat => rhs == RawNote::E,
            RawNote::E => rhs == RawNote::FFlat,

            RawNote::GFlat => rhs == RawNote::FSharp,
            RawNote::FSharp => rhs == RawNote::GFlat,

            RawNote::AFlat => rhs == RawNote::GSharp,
            RawNote::GSharp => rhs == RawNote::AFlat,

            RawNote::BFlat => rhs == RawNote::ASharp,
            RawNote::ASharp => rhs == RawNote::BFlat,

            RawNote::ESharp => rhs == RawNote::F,
            RawNote::F => rhs == RawNote::ESharp,

            RawNote::BSharp => rhs == RawNote::C,
            RawNote::C => rhs == RawNote::BSharp,
            _ => false,
        }
    }

    pub fn raw_note_to_basic_notation(raw_note: RawNote) -> &'static str {
        match raw_note {
            RawNote::Incongruent(_) => "Incongruent",
            RawNote::CFlat => "Cb",
            RawNote::C => "C",
            RawNote::CSharp => "C#",
            RawNote::DFlat => "Db",
            RawNote::D => "D",
            RawNote::DSharp => "D#",
            RawNote::EFlat => "Eb",
            RawNote::E => "E",
            RawNote::ESharp => "E#",
            RawNote::FFlat => "Fb",
            RawNote::F => "F",
            RawNote::FSharp => "F#",
            RawNote::GFlat => "Gb",
            RawNote::G => "G",
            RawNote::GSharp => "G#",
            RawNote::AFlat => "Ab",
            RawNote::A => "A",
            RawNote::ASharp => "A#",
            RawNote::BFlat => "Bb",
            RawNote::B => "B",
            RawNote::BSharp => "B#",
        }
    }

    pub fn raw_note_to_pretty_notation(raw_note: RawNote) -> String {
        match raw_note {
            RawNote::Incongruent(hz) => format!("({}hz)", hz),
            other => match other {
                RawNote::CFlat => "C\u{266D}",
                RawNote::C => "C",
                RawNote::CSharp => "C\u{266F}",
                RawNote::DFlat => "D\u{266D}",
                RawNote::D => "D",
                RawNote::DSharp => "D\u{266F}",
                RawNote::EFlat => "E\u{266D}",
                RawNote::E => "E",
                RawNote::ESharp => "E\u{266F}",
                RawNote::FFlat => "F\u{266D}",
                RawNote::F => "F",
                RawNote::FSharp => "F\u{266F}",
                RawNote::GFlat => "Gb",
                RawNote::G => "G",
                RawNote::GSharp => "G\u{266F}",
                RawNote::AFlat => "A\u{266D}",
                RawNote::A => "A",
                RawNote::ASharp => "A\u{266F}",
                RawNote::BFlat => "B\u{266D}",
                RawNote::B => "B",
                RawNote::BSharp => "B\u{266F}",
                RawNote::Incongruent(_) => unreachable!(),
            }
            .to_string(),
        }
    }

    pub fn raw_note_to_hz(raw_note: RawNote) -> Hertz {
        // I referenced https://pages.mtu.edu/~suits/notefreqs.html for the frequencies.

        // For notes C Flat and B Sharp, these are the same as B from the last octave and C from the next octave, respectively.
        //
        // Cb4 => B3
        // B#4 => C5
        //
        // To emulate this, I get the value from the previous octave, and apply one octave difference in the correct direction.
        // An octave is just the frequency doubled or halved.
        //
        // I chose to lay it out this way because bumping B4 up to a B4# conceptually is still in the octave 4, but logically
        // is the same frequency as a C5 in a different octave. I want to keep the conceptuality correct.

        // TODO: Implement the MIDI algorithm from https://newt.phys.unsw.edu.au/jw/notes.html instead!

        match raw_note {
            RawNote::Incongruent(hz) => hz,
            // C Flat is the same as B from the last octave
            RawNote::CFlat => RawNote::raw_note_to_hz(RawNote::B) / 2.0,
            RawNote::C => 16.35,
            RawNote::CSharp | RawNote::DFlat => 17.32,
            RawNote::D => 18.35,
            RawNote::DSharp | RawNote::EFlat => 19.45,
            RawNote::E | RawNote::FFlat => 20.60,
            RawNote::F | RawNote::ESharp => 21.83,
            RawNote::FSharp | RawNote::GFlat => 23.12,
            RawNote::G => 24.50,
            RawNote::GSharp | RawNote::AFlat => 25.96,
            RawNote::A => 27.50,
            RawNote::ASharp | RawNote::BFlat => 29.14,
            RawNote::B => 30.87,
            // B Sharp is the same as C from the next octave
            RawNote::BSharp => RawNote::raw_note_to_hz(RawNote::C) * 2.0,
        }
    }

    pub fn to_hertz(&self) -> Hertz {
        RawNote::raw_note_to_hz(*self)
    }

    pub fn to_basic_notation(&self) -> &str {
        RawNote::raw_note_to_basic_notation(*self)
    }

    pub fn to_pretty_notation(&self) -> String {
        RawNote::raw_note_to_pretty_notation(*self)
    }
}

pub const C_FLAT: RawNote = RawNote::CFlat;
pub const C: RawNote = RawNote::C;
pub const C_SHARP: RawNote = RawNote::CSharp;
pub const D_FLAT: RawNote = RawNote::DFlat;
pub const D: RawNote = RawNote::D;
pub const D_SHARP: RawNote = RawNote::DSharp;
pub const E_FLAT: RawNote = RawNote::EFlat;
pub const E: RawNote = RawNote::E;
pub const E_SHARP: RawNote = RawNote::ESharp;
pub const F_FLAT: RawNote = RawNote::FFlat;
pub const F: RawNote = RawNote::F;
pub const F_SHARP: RawNote = RawNote::FSharp;
pub const G_FLAT: RawNote = RawNote::GFlat;
pub const G: RawNote = RawNote::G;
pub const G_SHARP: RawNote = RawNote::GSharp;
pub const A_FLAT: RawNote = RawNote::AFlat;
pub const A: RawNote = RawNote::A;
pub const A_SHARP: RawNote = RawNote::ASharp;
pub const B_FLAT: RawNote = RawNote::BFlat;
pub const B: RawNote = RawNote::B;
pub const B_SHARP: RawNote = RawNote::BSharp;

pub struct IntoRawNoteError;

impl TryFrom<String> for RawNote {
    type Error = IntoRawNoteError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(PartialEq, Clone, Debug, Copy, Default)]
pub struct Note {
    raw_note: RawNote,
    octave: i16,
}

impl Note {
    pub fn new(raw_note: RawNote, octave: i16) -> Self {
        Note { raw_note, octave }
    }

    pub fn to_hertz(&self) -> Hertz {
        self.raw_note.to_hertz() * 2.0f32.powi(self.octave as i32)
    }

    pub fn octave(&self) -> i16 {
        self.octave
    }

    pub fn raw_note(&self) -> RawNote {
        self.raw_note
    }

    pub fn from_semitones_from_c0(semitones_from_low_c: u32) -> Note {
        let mut current_semitones = semitones_from_low_c;
        let mut octave = 0;
        let mut note = RawNote::C;

        while current_semitones >= 12 {
            current_semitones -= 12;
            octave += 1;
        }

        while current_semitones > 0 {
            current_semitones -= 1;
            note = match note {
                RawNote::C => RawNote::CSharp,
                RawNote::CSharp => RawNote::D,
                RawNote::D => RawNote::DSharp,
                RawNote::DSharp => RawNote::E,
                RawNote::E => RawNote::F,
                RawNote::F => RawNote::FSharp,
                RawNote::FSharp => RawNote::G,
                RawNote::G => RawNote::GSharp,
                RawNote::GSharp => RawNote::A,
                RawNote::A => RawNote::ASharp,
                RawNote::ASharp => RawNote::B,
                RawNote::B => RawNote::C,
                RawNote::CFlat => RawNote::B,
                RawNote::BFlat => RawNote::A,
                RawNote::EFlat => RawNote::D,
                RawNote::FFlat => RawNote::E,
                RawNote::GFlat => RawNote::F,
                RawNote::AFlat => RawNote::G,
                RawNote::DFlat => RawNote::C,
                RawNote::ESharp => RawNote::FSharp,
                RawNote::BSharp => RawNote::CSharp,
                RawNote::Incongruent(_) => unreachable!(),
            }
        }

        Note::new(note, octave as i16)
    }

    pub fn to_semitones_from_c0(&self) -> u32 {
        let mut semitones = 0;

        let mut current_octave = 0;
        let mut current_note = RawNote::C;

        while current_octave < self.octave {
            current_octave += 1;
        }

        while !current_note.is_analogous_to(&self.raw_note) {
            semitones += 1;
            current_note = match current_note {
                RawNote::C | RawNote::BSharp => RawNote::CSharp,
                RawNote::CSharp | RawNote::DFlat => RawNote::D,
                RawNote::D => RawNote::DSharp,
                RawNote::DSharp | RawNote::EFlat => RawNote::E,
                RawNote::E | RawNote::FFlat => RawNote::F,
                RawNote::F | RawNote::ESharp => RawNote::FSharp,
                RawNote::FSharp | RawNote::GFlat => RawNote::G,
                RawNote::G => RawNote::GSharp,
                RawNote::GSharp | RawNote::AFlat => RawNote::A,
                RawNote::A => RawNote::ASharp,
                RawNote::ASharp | RawNote::BFlat => RawNote::B,
                RawNote::B | RawNote::CFlat => RawNote::C,
                RawNote::Incongruent(_) => unreachable!(),
            }
        }

        semitones + (current_octave as u32 * 12)
    }

    pub fn add_semitones(&self, semitones: i32) -> Note {
        let new_semitones = self.to_semitones_from_c0() as i32 + semitones;

        if new_semitones < 0 {
            panic!("Cannot add semitones to a note that would result in a negative semitone value.")
        };

        Note::from_semitones_from_c0(new_semitones as u32)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            return Formatter::write_fmt(
                f,
                format_args!("{}{}", self.raw_note.to_basic_notation(), self.octave),
            );
        }

        Formatter::write_fmt(
            f,
            format_args!("{}{}", self.raw_note.to_pretty_notation(), self.octave),
        )
    }
}

impl Add for Note {
    type Output = Chord;

    fn add(self, rhs: Self) -> Self::Output {
        Chord::new(vec![self, rhs])
    }
}
