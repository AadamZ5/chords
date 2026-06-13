use std::num::ParseIntError;

use crate::try_from_string_prefix::TryFromStringPrefix;

pub type Octave = i32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntoOctaveError {
    EmptyInput,
    ParseIntError(ParseIntError),
    NonOctaveString(String),
}

impl TryFromStringPrefix for Octave {
    type Error = IntoOctaveError;

    fn try_from_string_prefix(value: &str) -> Result<(Self, &str), Self::Error> {
        if value.is_empty() {
            return Err(IntoOctaveError::EmptyInput);
        }

        // Find the last index of the end of the utf8 codepoint
        // for the last digit in the string.
        let last_digit_idx = value
            .char_indices()
            .take_while(|(i, c)| c.is_ascii_digit() || (*i == 0 && *c == '-'))
            .last()
            .map(|(i, c)| i + c.len_utf8());

        if let Some(last_digit_idx) = last_digit_idx {
            let (octave_str, remaining) = value.split_at(last_digit_idx);
            let octave = octave_str.parse().map_err(IntoOctaveError::ParseIntError)?;
            Ok((octave, remaining))
        } else {
            // If we didn't find any digits, and our string wasn't empty,
            // we must've had some non-digit characters
            Err(IntoOctaveError::NonOctaveString(value.to_string()))
        }
    }
}
