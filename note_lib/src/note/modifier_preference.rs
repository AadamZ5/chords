use super::NoteModifier;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ModifierPreference {
    Sharp,
    Flat,
}

impl From<NoteModifier> for ModifierPreference {
    fn from(modifier: NoteModifier) -> Self {
        match modifier {
            NoteModifier::Sharp | NoteModifier::DoubleSharp | NoteModifier::Natural => Self::Sharp,
            NoteModifier::Flat | NoteModifier::DoubleFlat => Self::Flat,
        }
    }
}
