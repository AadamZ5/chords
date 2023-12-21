use std::ops::Add;

use crate::Note;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Chord {
    notes: Vec<Note>,
}

impl Chord {
    pub fn new(notes: Vec<Note>) -> Self {
        Chord { notes }
    }

    pub fn notes(&self) -> &[Note] {
        &self.notes
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    pub fn set_notes(&mut self, notes: Vec<Note>) {
        self.notes = notes;
    }

    pub fn apply_inversion(&self, inversion: i8) -> Chord {
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
                    let note = Note::new(note.raw_note(), note.octave() - 1, note.modifier());
                    notes.insert(0, note);
                    inversion += 1;
                }
            }
            std::cmp::Ordering::Greater => {
                // Move notes up.
                while inversion > 0 {
                    let note = notes.remove(0);
                    // Move the note up an octave.
                    let note = Note::new(note.raw_note(), note.octave() + 1, note.modifier());
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
        Chord::new(Vec::from_iter(self.notes.iter().cloned().chain(rhs.notes)))
    }
}

impl Add<Note> for Chord {
    type Output = Self;

    fn add(self, rhs: Note) -> Self::Output {
        Chord::new(Vec::from_iter(
            self.notes.iter().cloned().chain(std::iter::once(rhs)),
        ))
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
