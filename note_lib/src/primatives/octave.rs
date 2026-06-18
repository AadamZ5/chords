use std::num::ParseIntError;

use thiserror::Error;

use crate::try_from_string_prefix::{
    try_integer_from_string_prefix, IntoIntegerError, TryFromStringPrefix,
};

pub type Octave = i32;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IntoOctaveError {
    #[error("Cannot parse empty string as an octave.")]
    EmptyInput,
    #[error("Failed to parse octave: {0}")]
    ParseIntError(ParseIntError),
    #[error("Invalid octave string: {0}")]
    NonOctaveString(String),
}

impl TryFromStringPrefix for Octave {
    type Error = IntoOctaveError;

    fn try_from_string_prefix(value: &str) -> Result<(Self, &str), Self::Error> {
        try_integer_from_string_prefix(value).map_err(|e| match e {
            IntoIntegerError::EmptyInput => IntoOctaveError::EmptyInput,
            IntoIntegerError::ParseIntError(parse_int_error) => {
                IntoOctaveError::ParseIntError(parse_int_error)
            }
            IntoIntegerError::NonDigitCharacters(non_digit_string) => {
                IntoOctaveError::NonOctaveString(non_digit_string)
            }
        })
    }
}
