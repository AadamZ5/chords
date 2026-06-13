pub trait TryFromStringPrefix: Sized {
    type Error;

    /// Tries to parse a string into the implementing type, but returning the remaining unused portion
    /// of the string if successful.
    fn try_from_string_prefix(value: &str) -> Result<(Self, &str), Self::Error>;
}
