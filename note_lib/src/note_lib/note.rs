use thiserror::Error;

use super::{AbstractNote, ModifierPreference, NoteModifier, RawNote};
use crate::{
    try_from_string_prefix::TryFromStringPrefix, Chord, CompoundInterval, Hertz, Interval,
    IntoModifierError, IntoOctaveError, IntoRawNoteError, Octave, Semitone, SimpleInterval,
    SimpleIntervalFromSemitones,
};
use std::{
    fmt::{Display, Formatter},
    ops::{Add, Sub},
    vec,
};

#[derive(PartialEq, Clone, Debug, Copy, Default)]
pub struct Note {
    abstract_note: AbstractNote,
    octave: Octave,
}

impl Note {
    pub fn new(raw_note: RawNote, octave: i32, modifier: NoteModifier) -> Self {
        Note {
            octave,
            abstract_note: AbstractNote { raw_note, modifier },
        }
    }

    pub fn to_hertz(&self) -> Hertz {
        // TODO: Modifiers are not accounted for here. We should probably convert semitones to hertz?
        self.abstract_note.raw_note.to_hertz() * 2.0f32.powi(self.octave)
    }

    pub fn octave(&self) -> Octave {
        self.octave
    }

    pub fn abstract_note(&self) -> AbstractNote {
        self.abstract_note
    }

    pub fn raw_note(&self) -> RawNote {
        self.abstract_note.raw_note
    }

    pub fn modifier(&self) -> NoteModifier {
        self.abstract_note.modifier
    }

    pub fn is_enharmonic_with(&self, other: &Note) -> bool {
        self.to_semitones_from_c0() == other.to_semitones_from_c0()
    }

    pub fn get_enharmonics(&self) -> impl Iterator<Item = Note> {
        let this_octave = self.octave;

        self.abstract_note
            .get_enharmonics()
            .map(move |abstract_note| abstract_note.at_octave(this_octave))
    }

    pub fn get_enharmonics_extended(&self) -> impl Iterator<Item = Note> {
        let this_octave = self.octave;

        self.abstract_note
            .get_enharmonics_extended()
            .map(move |abstract_note| abstract_note.at_octave(this_octave))
    }

    pub fn from_semitones_from_c0(
        semitones_from_low_c: Semitone,
        modifier_preference: ModifierPreference,
    ) -> Note {
        let SimpleIntervalFromSemitones {
            interval,
            mut octave_overflow,
        } = SimpleInterval::from_semitones(semitones_from_low_c);

        let abstract_note = match interval {
            // A perfect octave interval translates to a note in the next
            // octave. Since octaves aren't encoded in intervals, this is
            // missed. If it is a perfect octave, make it a perfect unison
            // in the next octave.
            SimpleInterval::PerfectOctave => {
                octave_overflow += 1;
                AbstractNote::from_interval_from_c(
                    SimpleInterval::PerfectUnison,
                    modifier_preference,
                )
            }
            _ => AbstractNote::from_interval_from_c(interval, modifier_preference),
        };

        abstract_note.at_octave(octave_overflow)
    }

    pub fn to_semitones_from_c0(&self) -> Semitone {
        let mut semitones_from_c = 0;

        let mut current_note = self.abstract_note.raw_note;
        let mut current_octave = 0;

        while current_octave < self.octave {
            current_octave += 1;
        }

        while current_note != RawNote::C {
            match current_note {
                RawNote::C => (),
                RawNote::Incongruent(_) => unreachable!(),
                _ => {
                    let (prev_note, semitones_to_prev_note) = current_note.prev_note();
                    current_note = prev_note;
                    semitones_from_c += semitones_to_prev_note;
                }
            }
        }

        let semitones_before_modified = (current_octave * 12) + semitones_from_c;

        semitones_before_modified + Semitone::from(self.abstract_note.modifier)
    }

    pub fn add_semitones(&self, semitones: Semitone) -> Note {
        let new_semitones = self.to_semitones_from_c0() + semitones;

        if new_semitones < 0 {
            panic!("Cannot add semitones to a note that would result in a negative semitone value from C0.")
        };

        Note::from_semitones_from_c0(new_semitones, self.abstract_note.modifier.into())
    }

    pub fn add_interval<T>(&self, interval: T) -> Note
    where
        T: Into<Interval>,
    {
        self.add_semitones(interval.into().semitones())
    }

    pub fn sub_interval<T>(&self, interval: T) -> Note
    where
        T: Into<Interval>,
    {
        self.add_semitones(-interval.into().semitones())
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Formatter::write_fmt(f, format_args!("{}{}", self.abstract_note, self.octave))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IntoNoteError {
    #[error("Cannot parse empty string as a note.")]
    EmptyNoteString,
    #[error("Invalid note string: {0}")]
    InvalidNoteString(String),
    #[error("Invalid pitch: {0}")]
    RawNoteParseError(IntoRawNoteError),
    #[error("Invalid note modifier: {0}")]
    NoteModifierParseError(IntoModifierError),
    #[error("Invalid octave: {0}")]
    OctaveParseError(IntoOctaveError),
}

impl TryFrom<&str> for Note {
    type Error = IntoNoteError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from_string_prefix(value).and_then(|(note, remaining)| {
            if remaining.is_empty() {
                Ok(note)
            } else {
                Err(IntoNoteError::InvalidNoteString(value.to_string()))
            }
        })
    }
}

impl TryFromStringPrefix for Note {
    type Error = IntoNoteError;

    fn try_from_string_prefix(value: &str) -> Result<(Self, &str), Self::Error> {
        let (raw_note, remaining) = RawNote::try_from_string_prefix(value)
            .map_err(|e| IntoNoteError::RawNoteParseError(e))?;

        // Since the modifier is optional, we only want to throw the modifier error if we can't successfully
        // also parse an octave.
        let modifier_result = NoteModifier::try_from_string_prefix(remaining)
            .map_err(|e| IntoNoteError::NoteModifierParseError(e));

        let (modifier, octave, remaining) = if let Ok((modifier, remaining)) = modifier_result {
            let (octave, remaining) = Octave::try_from_string_prefix(remaining)
                .map_err(|e| IntoNoteError::OctaveParseError(e))?;
            (modifier, octave, remaining)
        } else {
            // If we can't parse a modifier, see if we can further parse the octave. If we can parse an octave but not a modifier, that is ok.
            let octave_result = Octave::try_from_string_prefix(remaining)
                .map_err(|e| IntoNoteError::OctaveParseError(e));

            if let Ok((octave, remaining)) = octave_result {
                (NoteModifier::Natural, octave, remaining)
            } else {
                return Err(modifier_result.unwrap_err());
            }
        };

        let note = Note::new(raw_note, octave, modifier);
        Ok((note, remaining))
    }
}

impl Add for Note {
    type Output = Chord;

    fn add(self, rhs: Self) -> Self::Output {
        Chord::new(vec![self, rhs])
    }
}

impl Add<SimpleInterval> for Note {
    type Output = Note;

    fn add(self, rhs: SimpleInterval) -> Self::Output {
        self.add_interval(rhs)
    }
}

impl Sub<SimpleInterval> for Note {
    type Output = Note;

    fn sub(self, rhs: SimpleInterval) -> Self::Output {
        self.sub_interval(rhs)
    }
}

impl Add<CompoundInterval> for Note {
    type Output = Note;

    fn add(self, rhs: CompoundInterval) -> Self::Output {
        self.add_interval(rhs)
    }
}

impl Sub<CompoundInterval> for Note {
    type Output = Note;

    fn sub(self, rhs: CompoundInterval) -> Self::Output {
        self.sub_interval(rhs)
    }
}

impl Add<Interval> for Note {
    type Output = Note;

    fn add(self, rhs: Interval) -> Self::Output {
        self.add_interval(rhs)
    }
}

impl Sub<Interval> for Note {
    type Output = Note;

    fn sub(self, rhs: Interval) -> Self::Output {
        self.sub_interval(rhs)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_create() {
        let note = Note::new(RawNote::C, 4, NoteModifier::Natural);
        assert_eq!(note.raw_note(), RawNote::C);
        assert_eq!(note.octave(), 4);
        assert_eq!(note.modifier(), NoteModifier::Natural);
    }

    #[test]
    fn should_create_from_semitones() {
        // 12 up from C0 is C1.
        let semitones = 12;
        let note = Note::from_semitones_from_c0(semitones, ModifierPreference::Sharp);
        assert_eq!(note.raw_note(), RawNote::C);
        assert_eq!(note.octave(), 1);
        assert_eq!(note.modifier(), NoteModifier::Natural);

        let semitones = 12;
        let note = Note::from_semitones_from_c0(semitones, ModifierPreference::Flat);
        assert_eq!(note.raw_note(), RawNote::C);
        assert_eq!(note.octave(), 1);
        assert_eq!(note.modifier(), NoteModifier::Natural);

        let semitones = 13;
        let note = Note::from_semitones_from_c0(semitones, ModifierPreference::Sharp);
        assert_eq!(note.raw_note(), RawNote::C);
        assert_eq!(note.octave(), 1);
        assert_eq!(note.modifier(), NoteModifier::Sharp);

        let semitones = 13;
        let note = Note::from_semitones_from_c0(semitones, ModifierPreference::Flat);
        assert_eq!(note.raw_note(), RawNote::D);
        assert_eq!(note.octave(), 1);
        assert_eq!(note.modifier(), NoteModifier::Flat);

        let semitones = 14;
        let note = Note::from_semitones_from_c0(semitones, ModifierPreference::Flat);
        assert_eq!(note.raw_note(), RawNote::D);
        assert_eq!(note.octave(), 1);
        assert_eq!(note.modifier(), NoteModifier::Natural);
    }

    #[test]
    fn should_get_semitones() {
        let note = Note::new(RawNote::C, 4, NoteModifier::Natural);
        assert_eq!(note.to_semitones_from_c0(), 48);

        let note = Note::new(RawNote::C, 4, NoteModifier::Sharp);
        assert_eq!(note.to_semitones_from_c0(), 49);

        let note = Note::new(RawNote::C, 4, NoteModifier::Flat);
        assert_eq!(note.to_semitones_from_c0(), 47);
    }
}
