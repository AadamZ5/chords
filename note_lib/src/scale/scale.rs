use super::{ScaleMode, ScaleNoteIter};
use crate::AbstractNote;

/// Consider implementing scales.
///
/// Use:
/// - Scales represent a structure for notes to follow
/// - Scales have a root note
/// - Scales can be in a particular mode, which will produce a different pattern of notes
///

#[derive(Debug, Clone, PartialEq, Default)]
struct Scale {
    root_note: AbstractNote,
    mode: ScaleMode,
}

impl Scale {
    fn new(root_note: AbstractNote, mode: ScaleMode) -> Self {
        Self { root_note, mode }
    }
}

impl IntoIterator for Scale {
    type Item = AbstractNote;
    type IntoIter = ScaleNoteIter;

    fn into_iter(self) -> Self::IntoIter {
        ScaleNoteIter::new(self.root_note, self.mode)
    }
}
