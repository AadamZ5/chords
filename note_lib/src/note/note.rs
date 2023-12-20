use super::{AbstractNote, ModifierPreference, NoteModifier, RawNote};
use crate::{Chord, Hertz, Octave, Semitone, SimpleInterval, SimpleIntervalFromSemitones};
use std::{
    fmt::{Display, Formatter},
    ops::Add,
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
        self.abstract_note.raw_note.to_hertz() * 2.0f32.powi(self.octave as i32)
    }

    pub fn octave(&self) -> Octave {
        self.octave
    }

    pub fn raw_note(&self) -> RawNote {
        self.abstract_note.raw_note
    }

    pub fn modifier(&self) -> NoteModifier {
        self.abstract_note.modifier
    }

    pub fn from_semitones_from_c0(
        semitones_from_low_c: Semitone,
        modifier_preference: ModifierPreference,
    ) -> Note {
        let SimpleIntervalFromSemitones {
            interval,
            mut octave_overflow,
        } = SimpleInterval::from_semitones(semitones_from_low_c as i32);

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
        let semitones_modified =
            semitones_before_modified + Semitone::from(self.abstract_note.modifier);

        semitones_modified
    }

    pub fn add_semitones(&self, semitones: Semitone) -> Note {
        let new_semitones = self.to_semitones_from_c0() + semitones;

        if new_semitones < 0 {
            panic!("Cannot add semitones to a note that would result in a negative semitone value from C0.")
        };

        Note::from_semitones_from_c0(new_semitones, self.abstract_note.modifier.into())
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Formatter::write_fmt(f, format_args!("{}{}", self.abstract_note, self.octave))
    }
}

impl Add for Note {
    type Output = Chord;

    fn add(self, rhs: Self) -> Self::Output {
        Chord::new(vec![self, rhs])
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
