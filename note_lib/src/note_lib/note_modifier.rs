use std::fmt::Display;

use crate::try_from_string_prefix::TryFromStringPrefix;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, strum_macros::EnumIter)]
pub enum NoteModifier {
    DoubleFlat,
    Flat,
    #[default]
    Natural,
    Sharp,
    DoubleSharp,
    // TODO: How do we handle microtonal hoopla?
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntoModifierError {
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
