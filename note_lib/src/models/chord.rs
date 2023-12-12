use std::ops::Add;

use super::Note;

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
