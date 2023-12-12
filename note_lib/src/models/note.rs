use std::{
    fmt::{Display, Formatter},
    ops::Add,
    vec,
};

use crate::types::Hertz;

use super::Chord;

static STRING_INPUTS_TO_RAW_NOTE_MAP: [(RawNote, [&str; 0]); 0] = [];

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RawNote {
    /// A note that does not fit on the largely used 12-tone scale.
    Incongruent(Hertz),
    CFlat,
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

pub struct IntoRawNoteError;

impl TryFrom<String> for RawNote {
    type Error = IntoRawNoteError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}
#[derive(PartialEq, Clone, Debug, Copy)]
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
