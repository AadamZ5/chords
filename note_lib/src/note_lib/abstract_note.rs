use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::{Semitone, SimpleInterval};

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
    pub fn interval_from_c(&self) -> SimpleInterval {
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
            NoteModifier::DoubleFlat => -2,
        };

        SimpleInterval::from_semitones(semitones_from_c as i32 + modifier_semitone_adjustment)
            .interval
    }

    pub fn from_interval_from_c(
        interval: SimpleInterval,
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

    pub fn add_interval(&self, interval: SimpleInterval) -> Self {
        self.add_semitones(interval.semitones())
    }

    pub fn add_semitones(&self, semitones: Semitone) -> Self {
        if semitones == 0 {
            return *self;
        }

        let new_interval = self.interval_from_c().add_semitones(semitones).interval;
        bias_abstract_note_to_enharmonic_equivalent(
            &Self::from_interval_from_c(new_interval, self.modifier.into()),
            self.modifier,
        )
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

impl Add<NoteModifier> for AbstractNote {
    type Output = Self;

    fn add(self, rhs: NoteModifier) -> Self::Output {
        Self::from_interval_from_c(
            Add::<Semitone>::add(self.interval_from_c(), rhs.into()),
            rhs.into(),
        )
    }
}

impl Add<SimpleInterval> for AbstractNote {
    type Output = Self;

    fn add(self, rhs: SimpleInterval) -> Self::Output {
        self.add_interval(rhs)
    }
}

impl Sub<Semitone> for AbstractNote {
    type Output = Self;

    fn sub(self, rhs: Semitone) -> Self::Output {
        self.add_semitones(-rhs)
    }
}

impl Sub<NoteModifier> for AbstractNote {
    type Output = Self;

    fn sub(self, rhs: NoteModifier) -> Self::Output {
        Self::from_interval_from_c(
            Sub::<Semitone>::sub(self.interval_from_c(), rhs.into()),
            rhs.into(),
        )
    }
}

impl Sub<SimpleInterval> for AbstractNote {
    type Output = Self;

    fn sub(self, rhs: SimpleInterval) -> Self::Output {
        self.add_semitones(-rhs.semitones())
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
        if value.is_empty() {
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

// TODO: How do we handle when the bias is a double flat or double sharp,
// TODO: but the correct notation is a natural note? Somebody help us!
/// Converts a note to its enharmonic equivalent, given a bias. For example,
/// if the note is C# and the bias is flat, the enharmonic equivalent is Db.
/// If the note is C# and the bias is double flat, there is no enharmonic
/// equivalent, so C# is returned.
pub fn bias_abstract_note_to_enharmonic_equivalent(
    note: &AbstractNote,
    bias: NoteModifier,
) -> AbstractNote {
    // There MUST be some sort of rule we can implement
    // regarding how many half-steps / semitones to the next note,
    // and what the modifier can be.

    let mut current_note = note.raw_note;
    let mut semitone_acc: Semitone = 0;

    match note.modifier.cmp(&bias) {
        std::cmp::Ordering::Equal => *note,
        std::cmp::Ordering::Less => {
            // If our note is double flat, and we're searching for a note that's just flat,
            // (like trying to get from Fbb to Eb) then we're searching for a note that's lower.

            let mut semitones_to_prev_with_modifier_after_existing_modifier: Semitone;

            loop {
                let (prev_note, semitones_to_prev_note) = current_note.prev_note();
                semitones_to_prev_with_modifier_after_existing_modifier = semitones_to_prev_note
                    + Into::<Semitone>::into(note.modifier)
                    - Into::<Semitone>::into(bias)
                    + semitone_acc;

                if semitones_to_prev_with_modifier_after_existing_modifier == 0 {
                    break AbstractNote {
                        raw_note: prev_note,
                        modifier: bias,
                    };
                } else if semitones_to_prev_with_modifier_after_existing_modifier >= 0 {
                    break *note;
                } else {
                    current_note = prev_note;
                    semitone_acc += semitones_to_prev_note;
                }
            }
        }
        std::cmp::Ordering::Greater => {
            // If our note is sharp, and we're searching for a note that's double flat,
            // (like trying to get from D# to Fbb) then we're searching for a note that's higher.

            let mut semitones_to_next_with_modifier_after_existing_modifier: Semitone;

            loop {
                let (next_note, semitones_to_next_note) = current_note.next_note();
                semitones_to_next_with_modifier_after_existing_modifier = semitones_to_next_note
                    - Into::<Semitone>::into(note.modifier)
                    + Into::<Semitone>::into(bias)
                    - semitone_acc;

                if semitones_to_next_with_modifier_after_existing_modifier == 0 {
                    break AbstractNote {
                        raw_note: next_note,
                        modifier: bias,
                    };
                } else if semitones_to_next_with_modifier_after_existing_modifier <= 0 {
                    break *note;
                } else {
                    current_note = next_note;
                    semitone_acc += semitones_to_next_note;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use strum::IntoEnumIterator;

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
        let note = note + SimpleInterval::MajorSecond;
        assert_eq!(note.raw_note, RawNote::D);
        assert_eq!(note.modifier, NoteModifier::Natural);

        let note = note + SimpleInterval::MinorSecond;
        assert_eq!(note.raw_note, RawNote::D);
        assert_eq!(note.modifier, NoteModifier::Sharp);

        let note = note + SimpleInterval::PerfectOctave;
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

    #[test]
    fn add_intervals() {
        let note = AbstractNote {
            raw_note: RawNote::C,
            modifier: NoteModifier::Natural,
        };
        let note = note + SimpleInterval::MajorSecond + SimpleInterval::MinorSecond;
        assert_eq!(note.raw_note, RawNote::D);
        assert_eq!(note.modifier, NoteModifier::Sharp);

        let note = AbstractNote {
            raw_note: RawNote::C,
            modifier: NoteModifier::Sharp,
        };
        let note = note + SimpleInterval::MajorSecond + SimpleInterval::MinorSecond;
        assert_eq!(note.raw_note, RawNote::E);
        assert_eq!(note.modifier, NoteModifier::Natural);

        let note = AbstractNote {
            raw_note: RawNote::C,
            modifier: NoteModifier::Flat,
        };
        let note = note + SimpleInterval::MajorSecond + SimpleInterval::MinorSecond;
        assert_eq!(note.raw_note, RawNote::D);
        assert_eq!(note.modifier, NoteModifier::Natural);

        let note = AbstractNote {
            raw_note: RawNote::C,
            modifier: NoteModifier::Sharp,
        };
        let note = note + SimpleInterval::MajorSecond + SimpleInterval::MajorSecond;
        assert_eq!(note.raw_note, RawNote::E);
        assert_eq!(note.modifier, NoteModifier::Sharp);
    }

    #[test]
    fn enharmonic_modifier_bias() {
        let note = AbstractNote {
            raw_note: RawNote::F,
            modifier: NoteModifier::DoubleFlat,
        };
        assert_eq!(
            bias_abstract_note_to_enharmonic_equivalent(&note, NoteModifier::Sharp),
            AbstractNote {
                raw_note: RawNote::D,
                modifier: NoteModifier::Sharp
            }
        );
        let note = AbstractNote {
            raw_note: RawNote::F,
            modifier: NoteModifier::Flat,
        };
        assert_eq!(
            bias_abstract_note_to_enharmonic_equivalent(&note, NoteModifier::Natural),
            AbstractNote {
                raw_note: RawNote::E,
                modifier: NoteModifier::Natural
            }
        );
        let note = AbstractNote {
            raw_note: RawNote::D,
            modifier: NoteModifier::Sharp,
        };
        assert_eq!(
            bias_abstract_note_to_enharmonic_equivalent(&note, NoteModifier::Flat),
            AbstractNote {
                raw_note: RawNote::E,
                modifier: NoteModifier::Flat
            }
        );

        let mut notes: Vec<AbstractNote> = Vec::new();

        for note in RawNote::iter().filter(|n| !matches!(n, RawNote::Incongruent(_))) {
            for modifier in NoteModifier::iter() {
                notes.push(AbstractNote {
                    raw_note: note,
                    modifier,
                })
            }
        }

        for note in notes {
            for modifier_bias in NoteModifier::iter() {
                let biased_note = bias_abstract_note_to_enharmonic_equivalent(&note, modifier_bias);
                println!("{:?}, {:?}", note, biased_note);

                let note_interval = note.interval_from_c();
                let biased_note_interval = biased_note.interval_from_c();

                // Since our system uses C as a basis for the interval, we should consider a perfect octave
                // and a perfect unison the same since in this abstract representation, they are enharmonically
                // the same. For ex, B# and C are the same note, but B# is technically a perfect octave above C.
                let similar_interval = match note_interval {
                    SimpleInterval::PerfectUnison | SimpleInterval::PerfectOctave => {
                        match biased_note_interval {
                            SimpleInterval::PerfectUnison | SimpleInterval::PerfectOctave => true,
                            _ => false,
                        }
                    }
                    _ => note_interval == biased_note_interval,
                };

                assert!(similar_interval)
            }
        }
    }
}
