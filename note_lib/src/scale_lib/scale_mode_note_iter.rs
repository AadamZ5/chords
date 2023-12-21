use strum::IntoEnumIterator;

use crate::AbstractNote;

use super::{ScaleDegree, ScaleDegreeIter, ScaleMode};

#[derive(Debug)]
pub struct ScaleNoteIter {
    root: AbstractNote,
    mode: ScaleMode,
    current_degree: ScaleDegreeIter,
}

impl ScaleNoteIter {
    pub fn new(root: AbstractNote, mode: ScaleMode) -> Self {
        Self {
            root,
            mode,
            current_degree: ScaleDegree::iter(),
        }
    }
}

impl Iterator for ScaleNoteIter {
    type Item = AbstractNote;

    fn next(&mut self) -> Option<Self::Item> {
        let next_degree = self.current_degree.next()?;
        let next_note = self.root + self.mode.interval_at_degree(next_degree);
        Some(next_note)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn scale_mode_iterates() {
        let root = AbstractNote::try_from("C").unwrap();
        let mode = ScaleMode::Ionian;
        let scale_mode_iter = ScaleNoteIter::new(root, mode);
        let scale: Vec<AbstractNote> = scale_mode_iter.take(8).collect();
        assert_eq!(
            scale,
            vec![
                AbstractNote::try_from("C").unwrap(),
                AbstractNote::try_from("D").unwrap(),
                AbstractNote::try_from("E").unwrap(),
                AbstractNote::try_from("F").unwrap(),
                AbstractNote::try_from("G").unwrap(),
                AbstractNote::try_from("A").unwrap(),
                AbstractNote::try_from("B").unwrap(),
                AbstractNote::try_from("C").unwrap(),
            ]
        );
    }
}
