use std::{fmt::Display, ops::Add};

use crate::{Interval, Semitone};

use super::{ModifierPreference, Note, NoteModifier, RawNote};

/// Represents a note that has a modifier, but no octave defined.
/// This is typically used when talking about [`super::super::ScaleMode`]s
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AbstractNote {
    pub raw_note: RawNote,
    pub modifier: NoteModifier,
}

impl AbstractNote {
    /// Creates an octave-placed note using this note's raw note and modifier.
    pub fn at_octave(&self, octave: i32) -> Note {
        Note::new(self.raw_note, octave, self.modifier)
    }

    /// Gets the abstract note's interval from C
    pub fn interval_from_c(&self) -> Interval {
        let mut semitones_from_c = 0;
        let mut current_note = self.raw_note;
        while current_note != RawNote::C {
            match current_note {
                RawNote::C => (),
                RawNote::Incongruent(_) => unreachable!(),
                _ => {
                    let (prev_note, semitones_to_prev_note) = current_note.prev_note();
                    current_note = prev_note;
                    semitones_from_c += semitones_to_prev_note as u32;
                }
            }
        }

        let modifier_semitone_adjustment = match self.modifier {
            NoteModifier::Sharp => 1,
            NoteModifier::Flat => -1,
            NoteModifier::Natural => 0,
            NoteModifier::DoubleSharp => 2,
            NoteModifier::DoubleFlat => 2,
        };

        Interval::from_semitone_interval(
            (semitones_from_c as i32 + modifier_semitone_adjustment) as i32,
        )
        .interval
    }

    pub fn from_interval_from_c(
        interval: Interval,
        modifier_preference: ModifierPreference,
    ) -> Self {
        let mut current_semitones = interval.semitones();
        let mut note = RawNote::C;
        let mut modifier = NoteModifier::Natural;

        while current_semitones > 0 {
            let (next_note, semitones_to_next_note) = note.next_note();

            if current_semitones >= semitones_to_next_note {
                note = next_note;
                current_semitones -= semitones_to_next_note;
            } else if current_semitones == 1 {
                if modifier_preference == ModifierPreference::Sharp {
                    modifier = NoteModifier::Sharp;
                } else {
                    note = next_note;
                    modifier = NoteModifier::Flat;
                }
                current_semitones -= 1;
            }
        }

        Self {
            raw_note: note,
            modifier,
        }
    }

    pub fn add_interval(&self, interval: Interval) -> Self {
        self.add_semitones(interval.semitones())
    }

    pub fn add_semitones(&self, semitones: Semitone) -> Self {
        if semitones == 0 {
            return *self;
        }

        let new_interval = self.interval_from_c().add_semitones(semitones).interval;
        Self::from_interval_from_c(new_interval, self.modifier.into())
    }
}

impl Display for AbstractNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.raw_note, self.modifier)
    }
}

impl Add<Semitone> for AbstractNote {
    type Output = Self;

    fn add(self, rhs: Semitone) -> Self::Output {
        self.add_semitones(rhs)
    }
}

impl Add<Interval> for AbstractNote {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        self.add_interval(rhs)
    }
}

impl From<Note> for AbstractNote {
    fn from(note: Note) -> Self {
        Self {
            raw_note: note.raw_note(),
            modifier: note.modifier(),
        }
    }
}

impl From<RawNote> for AbstractNote {
    fn from(raw_note: RawNote) -> Self {
        Self {
            raw_note,
            modifier: NoteModifier::Natural,
        }
    }
}

impl From<(RawNote, NoteModifier)> for AbstractNote {
    fn from((raw_note, modifier): (RawNote, NoteModifier)) -> Self {
        Self { raw_note, modifier }
    }
}

impl From<(NoteModifier, RawNote)> for AbstractNote {
    fn from((modifier, raw_note): (NoteModifier, RawNote)) -> Self {
        Self { raw_note, modifier }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbstractNoteParseError {
    EmptyInput,
    InvalidNote,
    InvalidModifier,
    InputTooLong,
}

impl TryFrom<String> for AbstractNote {
    type Error = AbstractNoteParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 0 {
            return Err(AbstractNoteParseError::EmptyInput);
        }

        let trimmed = value.trim();
        // The trimmed value shouldn't be more than 3 characters.
        if trimmed.len() > 3 {
            return Err(AbstractNoteParseError::InputTooLong);
        }

        let (first, rest) = trimmed.split_at(1);

        let raw_note = match first.to_ascii_uppercase().as_str() {
            "A" => RawNote::A,
            "B" => RawNote::B,
            "C" => RawNote::C,
            "D" => RawNote::D,
            "E" => RawNote::E,
            "F" => RawNote::F,
            "G" => RawNote::G,
            _ => return Err(AbstractNoteParseError::InvalidNote),
        };

        let modifier = match rest {
            "" => NoteModifier::Natural,
            "#" => NoteModifier::Sharp,
            "b" => NoteModifier::Flat,
            "##" | "x" => NoteModifier::DoubleSharp,
            "bb" => NoteModifier::DoubleFlat,
            _ => return Err(AbstractNoteParseError::InvalidModifier),
        };

        Ok(Self { raw_note, modifier })
    }
}

impl TryFrom<&str> for AbstractNote {
    type Error = AbstractNoteParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_create() {
        let note = AbstractNote {
            raw_note: RawNote::C,
            modifier: NoteModifier::Natural,
        };
        assert_eq!(note.raw_note, RawNote::C);
        assert_eq!(note.modifier, NoteModifier::Natural);
    }

    #[test]
    fn should_create_from_note() {
        let note = Note::new(RawNote::C, 4, NoteModifier::Natural);
        let abstract_note: AbstractNote = note.into();
        assert_eq!(abstract_note.raw_note, RawNote::C);
        assert_eq!(abstract_note.modifier, NoteModifier::Natural);
    }

    #[test]
    fn should_create_from_raw_note() {
        let note = RawNote::C;
        let abstract_note: AbstractNote = note.into();
        assert_eq!(abstract_note.raw_note, RawNote::C);
        assert_eq!(abstract_note.modifier, NoteModifier::Natural);
    }

    #[test]
    fn should_add_interval() {
        let note = AbstractNote {
            raw_note: RawNote::C,
            modifier: NoteModifier::Natural,
        };
        let note = note + Interval::MajorSecond;
        assert_eq!(note.raw_note, RawNote::D);
        assert_eq!(note.modifier, NoteModifier::Natural);

        let note = note + Interval::MinorSecond;
        assert_eq!(note.raw_note, RawNote::D);
        assert_eq!(note.modifier, NoteModifier::Sharp);

        let note = note + Interval::PerfectOctave;
        assert_eq!(note.raw_note, RawNote::D);
        assert_eq!(note.modifier, NoteModifier::Sharp);
    }

    #[test]
    fn should_add_semitones() {
        let note = AbstractNote {
            raw_note: RawNote::C,
            modifier: NoteModifier::Natural,
        };
        let note = note + 2;
        assert_eq!(note.raw_note, RawNote::D);
        assert_eq!(note.modifier, NoteModifier::Natural);
    }

    #[test]
    fn to_and_from_semitones() {
        let abstract_notes = vec![
            AbstractNote {
                raw_note: RawNote::C,
                modifier: NoteModifier::Natural,
            },
            AbstractNote {
                raw_note: RawNote::C,
                modifier: NoteModifier::Sharp,
            },
            AbstractNote {
                raw_note: RawNote::C,
                modifier: NoteModifier::Flat,
            },
        ];

        for abstract_note in abstract_notes {
            let interval = abstract_note.interval_from_c();
            println!("{:?}", interval);
            let semitones = interval.semitones();
            let new_abstract_note = AbstractNote::from_interval_from_c(
                abstract_note.interval_from_c(),
                abstract_note.modifier.into(),
            );
            // We can't assert the notes are equal, because we lose information when
            // transforming to intervals (and semitones) because some notes can be
            // represented by another note with a different modifier.
            // For example, C# and Db are the same note, but they are represented
            // differently, but mean the same pitch.
            //assert_eq!(new_abstract_note, abstract_note);
            assert_eq!(new_abstract_note.interval_from_c().semitones(), semitones);
        }
    }
}
