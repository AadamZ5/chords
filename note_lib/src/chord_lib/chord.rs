use std::ops::Add;

use crate::{Note, SimpleInterval};

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Chord {
    notes: Vec<Note>,
    // TODO: Do we need to preserve the intended root and quality?
}

impl Chord {
    pub fn new<T>(notes: T) -> Self
    where
        T: IntoIterator<Item = Note>,
    {
        Chord {
            notes: notes.into_iter().collect(),
        }
    }

    pub fn notes(&self) -> &[Note] {
        &self.notes
    }

    /// Adds a note to the chord. This does not order the notes in any way.
    /// If you require a specific order for inversions, you should use [Chord::set_notes] instead.
    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    /// Sets the notes for this chord. This replaces all existing notes in the chord with the provided notes.
    /// Note that the root note of the chord is assumed as the first note in the provided notes.
    pub fn set_notes<T>(&mut self, notes: T)
    where
        T: IntoIterator<Item = Note>,
    {
        self.notes = notes.into_iter().collect();
    }

    /// Is this chord empty? A chord is empty if it has no notes.
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// Applies an inversion to the chord.
    ///
    /// Inversions take the lowest note of the chord and move it up an octave,
    /// or take the highest note of the chord and move it down an octave,
    /// depending on the direction of the inversion.
    ///
    /// For example, if we have a C major chord, C4 E4 G4, and we apply a positive inversion,
    /// we get E4 G4 C5. If we apply a negative inversion, we get G3 C4 E4.
    ///
    /// Note that taking another inversion of an already inverted chord is not necessarily the same as
    /// taking a higher inversion of the original chord.
    pub fn apply_inversion(&self, inversion: i8) -> Chord {
        if self.notes.is_empty() {
            return self.clone();
        }

        // When a positive inversion happens, we move the lowest note of the chord up one octave.
        // For a negative inversion for programming convenience, we just do the opposite.
        // For example, if we have a C major chord, C4 E4 G4, and we apply a positive inversion,
        // we get E4 G4 C5. If we apply a negative inversion, we get G3 C4 E4.

        let mut notes = self.notes.clone();
        let mut inversion = inversion;

        match inversion.cmp(&0) {
            std::cmp::Ordering::Equal => {
                // Do nothing. Leave notes as-is.
            }
            std::cmp::Ordering::Less => {
                // Move notes down.
                while inversion < 0 {
                    let note = notes.remove(notes.len() - 1);
                    // Move the note down an octave.
                    let note = note - SimpleInterval::PerfectOctave;
                    notes.insert(0, note);
                    inversion += 1;
                }
            }
            std::cmp::Ordering::Greater => {
                // Move notes up.
                while inversion > 0 {
                    let note = notes.remove(0);
                    // Move the note up an octave.
                    let note = note + SimpleInterval::PerfectOctave;
                    notes.push(note);
                    inversion -= 1
                }
            }
        };
        Chord::new(notes)
    }
}

impl Add for Chord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Chord::new(self.notes.iter().cloned().chain(rhs.notes))
    }
}

impl Add<Note> for Chord {
    type Output = Self;

    fn add(self, rhs: Note) -> Self::Output {
        Chord::new(self.notes.iter().cloned().chain(std::iter::once(rhs)))
    }
}

#[cfg(test)]
mod tests {

    use crate::{NoteModifier, B, C, E, G};

    use super::*;

    #[test]
    fn a_chord_is_created() {
        let chord = Chord::new(vec![
            Note::new(C, 4, NoteModifier::Natural),
            Note::new(E, 4, NoteModifier::Natural),
            Note::new(G, 4, NoteModifier::Natural),
        ]);
        assert_eq!(
            chord.notes,
            vec![
                Note::new(C, 4, NoteModifier::Natural),
                Note::new(E, 4, NoteModifier::Natural),
                Note::new(G, 4, NoteModifier::Natural)
            ]
        );
    }

    #[test]
    fn chord_note_is_added() {
        let chord = Chord::new(vec![
            Note::new(C, 4, NoteModifier::Natural),
            Note::new(E, 4, NoteModifier::Natural),
            Note::new(G, 4, NoteModifier::Natural),
        ]);

        let add_7 = chord + Note::new(B, 4, NoteModifier::Natural);
        assert_eq!(
            add_7.notes,
            vec![
                Note::new(C, 4, NoteModifier::Natural),
                Note::new(E, 4, NoteModifier::Natural),
                Note::new(G, 4, NoteModifier::Natural),
                Note::new(B, 4, NoteModifier::Natural)
            ]
        );
    }

    #[test]
    fn inversion_is_applied() {
        let initial_chord = Chord::new(vec![
            Note::new(C, 4, NoteModifier::Natural),
            Note::new(E, 4, NoteModifier::Natural),
            Note::new(G, 4, NoteModifier::Natural),
        ]);

        let first_inversion = initial_chord.apply_inversion(1);
        assert_eq!(
            first_inversion.notes,
            vec![
                Note::new(E, 4, NoteModifier::Natural),
                Note::new(G, 4, NoteModifier::Natural),
                Note::new(C, 5, NoteModifier::Natural)
            ]
        );

        let second_inversion = initial_chord.apply_inversion(2);
        assert_eq!(
            second_inversion.notes,
            vec![
                Note::new(G, 4, NoteModifier::Natural),
                Note::new(C, 5, NoteModifier::Natural),
                Note::new(E, 5, NoteModifier::Natural)
            ]
        );

        let zero_inversion = initial_chord.apply_inversion(0);
        assert_eq!(
            zero_inversion.notes,
            vec![
                Note::new(C, 4, NoteModifier::Natural),
                Note::new(E, 4, NoteModifier::Natural),
                Note::new(G, 4, NoteModifier::Natural)
            ]
        );

        let negative_inversion = initial_chord.apply_inversion(-1);
        assert_eq!(
            negative_inversion.notes,
            vec![
                Note::new(G, 3, NoteModifier::Natural),
                Note::new(C, 4, NoteModifier::Natural),
                Note::new(E, 4, NoteModifier::Natural)
            ]
        );
    }
}
