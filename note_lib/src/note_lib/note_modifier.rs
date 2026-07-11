use std::fmt::Display;

use crate::try_from_string_prefix::TryFromStringPrefix;
use strum::IntoEnumIterator;
use thiserror::Error;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, strum_macros::EnumIter, Hash,
)]
pub enum NoteModifier {
    DoubleFlat,
    Flat,
    #[default]
    Natural,
    Sharp,
    DoubleSharp,
    // TODO: How do we handle microtonal hoopla?
}

impl NoteModifier {
    pub fn get_semitone_adjustment(&self) -> i32 {
        match self {
            NoteModifier::DoubleFlat => -2,
            NoteModifier::Flat => -1,
            NoteModifier::Natural => 0,
            NoteModifier::Sharp => 1,
            NoteModifier::DoubleSharp => 2,
        }
    }

    pub fn iter_all_modifiers() -> NoteModifierIter {
        Self::iter()
    }

    pub fn iter_common_modifiers() -> impl Iterator<Item = NoteModifier> {
        Self::iter().filter(|m| {
            matches!(
                m,
                NoteModifier::Flat | NoteModifier::Natural | NoteModifier::Sharp
            )
        })
    }
}

impl Display for NoteModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let modifier = match self {
            NoteModifier::Sharp => "#",
            NoteModifier::Flat => "b",
            NoteModifier::Natural => "",
            NoteModifier::DoubleSharp => "##",
            NoteModifier::DoubleFlat => "bb",
        };

        write!(f, "{}", modifier)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IntoModifierError {
    #[error("Cannot parse empty string as a note modifier.")]
    EmptyInput,
    #[error("Unknown note modifier: {0}")]
    UnknownModifier(String),
}

impl TryFrom<&str> for NoteModifier {
    type Error = IntoModifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from_string_prefix(value).and_then(|(modifier, remaining)| {
            if remaining.is_empty() {
                Ok(modifier)
            } else {
                Err(IntoModifierError::UnknownModifier(value.to_string()))
            }
        })
    }
}

impl TryFromStringPrefix for NoteModifier {
    type Error = IntoModifierError;

    fn try_from_string_prefix(value: &str) -> Result<(Self, &str), Self::Error> {
        if value.is_empty() {
            return Err(IntoModifierError::EmptyInput);
        }

        let modifier = if value.starts_with("##") {
            NoteModifier::DoubleSharp
        } else if value.starts_with("bb") {
            NoteModifier::DoubleFlat
        } else if value.starts_with('#') {
            NoteModifier::Sharp
        } else if value.starts_with('b') {
            NoteModifier::Flat
        } else {
            return Err(IntoModifierError::UnknownModifier(value.to_string()));
        };

        let remaining = &value[modifier.to_string().len()..];
        Ok((modifier, remaining))
    }
}
