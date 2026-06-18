pub trait TryFromStringPrefix: Sized {
    type Error;

    /// Tries to parse a string into the implementing type, but returning the remaining unused portion
    /// of the string if successful.
    fn try_from_string_prefix(value: &str) -> Result<(Self, &str), Self::Error>;
}

pub enum IntoIntegerError {
    EmptyInput,
    NonDigitCharacters(String),
    ParseIntError(std::num::ParseIntError),
}

pub fn try_integer_from_string_prefix(value: &str) -> Result<(i32, &str), IntoIntegerError> {
    if value.is_empty() {
        return Err(IntoIntegerError::EmptyInput);
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
        let octave = octave_str
            .parse()
            .map_err(IntoIntegerError::ParseIntError)?;
        Ok((octave, remaining))
    } else {
        // If we didn't find any digits, and our string wasn't empty,
        // we must've had some non-digit characters
        Err(IntoIntegerError::NonDigitCharacters(value.to_string()))
    }
}
