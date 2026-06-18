use std::ops::{Add, Sub};

use crate::{
    try_from_string_prefix::TryFromStringPrefix, IntervalQuality, OtherCompoundInterval, Semitone,
    SimpleIntervalFromSemitones,
};

/// Simple interval numbers represent the "rank" of the interval. For example,
/// a minor third and a major third both have an interval number of "third". The
/// interval number is used in conjunction with the interval quality to determine
/// the specific interval. For example, a minor third is a third with a minor quality.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum_macros::EnumIter,
    strum_macros::Display,
)]
pub enum SimpleIntervalNumber {
    Unison = 1,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave,
}

pub enum IntoSimpleIntervalNumberError<T> {
    NoMatch(T),
}

impl TryFrom<u8> for SimpleIntervalNumber {
    type Error = IntoSimpleIntervalNumberError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let interval = match value {
            1 => SimpleIntervalNumber::Unison,
            2 => SimpleIntervalNumber::Second,
            3 => SimpleIntervalNumber::Third,
            4 => SimpleIntervalNumber::Fourth,
            5 => SimpleIntervalNumber::Fifth,
            6 => SimpleIntervalNumber::Sixth,
            7 => SimpleIntervalNumber::Seventh,
            8 => SimpleIntervalNumber::Octave,
            _ => return Err(IntoSimpleIntervalNumberError::NoMatch(value)),
        };

        Ok(interval)
    }
}

impl Into<u8> for SimpleIntervalNumber {
    fn into(self) -> u8 {
        self as u8
    }
}

/// If the [OtherCompoundInterval] is just composed of one simple interval (when simplified),
/// then extract that simple interval.
impl TryFrom<OtherCompoundInterval> for SimpleInterval {
    type Error = ();

    fn try_from(compound_interval: OtherCompoundInterval) -> Result<Self, Self::Error> {
        let simplified = compound_interval.simplify();
        if simplified.stack().len() == 1 {
            Ok(simplified.stack()[0])
        } else {
            Err(())
        }
    }
}

impl TryFrom<&OtherCompoundInterval> for SimpleInterval {
    type Error = ();

    fn try_from(compound_interval: &OtherCompoundInterval) -> Result<Self, Self::Error> {
        let simplified = compound_interval.simplify();
        if simplified.stack().len() == 1 {
            Ok(simplified.stack()[0])
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidSimpleIntervalError {
    InvalidPerfectNumber,
    InvalidAugmentedNumber,
    InvalidDiminishedNumber,
    InvalidMajorNumber,
    InvalidMinorNumber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum_macros::EnumIter)]
pub enum SimpleInterval {
    // These are listed in order of smallest to largest, beyond just
    // enharmonic equivalence. For example, a minor third is conceptually
    // "larger" than an augmented second.
    PerfectUnison,
    DiminishedSecond,

    AugmentedUnison,
    MinorSecond,

    MajorSecond,
    DiminishedThird,

    AugmentedSecond,
    MinorThird,

    MajorThird,
    DiminishedFourth,

    AugmentedThird,
    PerfectFourth,

    DiminishedFifth,
    AugmentedFourth,

    PerfectFifth,
    DiminishedSixth,

    AugmentedFifth,
    MinorSixth,

    MajorSixth,
    DiminishedSeventh,

    AugmentedSixth,
    MinorSeventh,

    MajorSeventh,
    DiminishedOctave,

    AugmentedSeventh,
    PerfectOctave,
}

/// Represents a musical interval. Intervals are used for measuring the distance
/// from a lower note to a higher note within a single octave span.
impl SimpleInterval {
    /// Given an interval number and quality like [`IntervalQuality::Major`]
    /// and [`IntervalNumber::Third`], try to match to a valid interval enum.
    /// This is falliable because some combinations of interval number and
    /// quality are invalid. For example, there is no such thing as a major unison,
    /// or a perfect third.
    ///
    /// ```rust
    /// use note_lib::{SimpleInterval, SimpleIntervalNumber, IntervalQuality};
    ///
    /// let result = SimpleInterval::from_quality_and_number(IntervalQuality::Major, SimpleIntervalNumber::Third);
    /// assert_eq!(result, Ok(SimpleInterval::MajorThird));
    ///
    /// let result = SimpleInterval::from_quality_and_number(IntervalQuality::Perfect, SimpleIntervalNumber::Third);
    /// assert_eq!(result, Err(note_lib::InvalidSimpleIntervalError::InvalidPerfectNumber));
    /// ```
    pub fn from_quality_and_number(
        quality: IntervalQuality,
        interval_number: SimpleIntervalNumber,
    ) -> Result<SimpleInterval, InvalidSimpleIntervalError> {
        match (interval_number, quality) {
            (SimpleIntervalNumber::Unison, IntervalQuality::Perfect) => {
                Ok(SimpleInterval::PerfectUnison)
            }
            (SimpleIntervalNumber::Unison, IntervalQuality::Major) => {
                Err(InvalidSimpleIntervalError::InvalidMajorNumber)
            }
            (SimpleIntervalNumber::Unison, IntervalQuality::Minor) => {
                Err(InvalidSimpleIntervalError::InvalidMinorNumber)
            }
            (SimpleIntervalNumber::Unison, IntervalQuality::Augmented) => {
                Ok(SimpleInterval::AugmentedUnison)
            }
            (SimpleIntervalNumber::Unison, IntervalQuality::Diminished) => {
                Err(InvalidSimpleIntervalError::InvalidDiminishedNumber)
            }
            (SimpleIntervalNumber::Second, IntervalQuality::Perfect) => {
                Err(InvalidSimpleIntervalError::InvalidPerfectNumber)
            }
            (SimpleIntervalNumber::Second, IntervalQuality::Major) => {
                Ok(SimpleInterval::MajorSecond)
            }
            (SimpleIntervalNumber::Second, IntervalQuality::Minor) => {
                Ok(SimpleInterval::MinorSecond)
            }
            (SimpleIntervalNumber::Second, IntervalQuality::Augmented) => {
                Ok(SimpleInterval::AugmentedSecond)
            }
            (SimpleIntervalNumber::Second, IntervalQuality::Diminished) => {
                Ok(SimpleInterval::DiminishedSecond)
            }
            (SimpleIntervalNumber::Third, IntervalQuality::Perfect) => {
                Err(InvalidSimpleIntervalError::InvalidPerfectNumber)
            }
            (SimpleIntervalNumber::Third, IntervalQuality::Major) => Ok(SimpleInterval::MajorThird),
            (SimpleIntervalNumber::Third, IntervalQuality::Minor) => Ok(SimpleInterval::MinorThird),
            (SimpleIntervalNumber::Third, IntervalQuality::Augmented) => {
                Ok(SimpleInterval::AugmentedThird)
            }
            (SimpleIntervalNumber::Third, IntervalQuality::Diminished) => {
                Ok(SimpleInterval::DiminishedThird)
            }
            (SimpleIntervalNumber::Fourth, IntervalQuality::Perfect) => {
                Ok(SimpleInterval::PerfectFourth)
            }
            (SimpleIntervalNumber::Fourth, IntervalQuality::Major) => {
                Err(InvalidSimpleIntervalError::InvalidMajorNumber)
            }
            (SimpleIntervalNumber::Fourth, IntervalQuality::Minor) => {
                Err(InvalidSimpleIntervalError::InvalidMinorNumber)
            }
            (SimpleIntervalNumber::Fourth, IntervalQuality::Augmented) => {
                Ok(SimpleInterval::AugmentedFourth)
            }
            (SimpleIntervalNumber::Fourth, IntervalQuality::Diminished) => {
                Ok(SimpleInterval::DiminishedFourth)
            }
            (SimpleIntervalNumber::Fifth, IntervalQuality::Perfect) => {
                Ok(SimpleInterval::PerfectFifth)
            }
            (SimpleIntervalNumber::Fifth, IntervalQuality::Major) => {
                Err(InvalidSimpleIntervalError::InvalidMajorNumber)
            }
            (SimpleIntervalNumber::Fifth, IntervalQuality::Minor) => {
                Err(InvalidSimpleIntervalError::InvalidMinorNumber)
            }
            (SimpleIntervalNumber::Fifth, IntervalQuality::Augmented) => {
                Ok(SimpleInterval::AugmentedFifth)
            }
            (SimpleIntervalNumber::Fifth, IntervalQuality::Diminished) => {
                Ok(SimpleInterval::DiminishedFifth)
            }
            (SimpleIntervalNumber::Sixth, IntervalQuality::Perfect) => {
                Err(InvalidSimpleIntervalError::InvalidPerfectNumber)
            }
            (SimpleIntervalNumber::Sixth, IntervalQuality::Major) => Ok(SimpleInterval::MajorSixth),
            (SimpleIntervalNumber::Sixth, IntervalQuality::Minor) => Ok(SimpleInterval::MinorSixth),
            (SimpleIntervalNumber::Sixth, IntervalQuality::Augmented) => {
                Ok(SimpleInterval::AugmentedSixth)
            }
            (SimpleIntervalNumber::Sixth, IntervalQuality::Diminished) => {
                Ok(SimpleInterval::DiminishedSixth)
            }
            (SimpleIntervalNumber::Seventh, IntervalQuality::Perfect) => {
                Err(InvalidSimpleIntervalError::InvalidPerfectNumber)
            }
            (SimpleIntervalNumber::Seventh, IntervalQuality::Major) => {
                Ok(SimpleInterval::MajorSeventh)
            }
            (SimpleIntervalNumber::Seventh, IntervalQuality::Minor) => {
                Ok(SimpleInterval::MinorSeventh)
            }
            (SimpleIntervalNumber::Seventh, IntervalQuality::Augmented) => {
                Ok(SimpleInterval::AugmentedSeventh)
            }
            (SimpleIntervalNumber::Seventh, IntervalQuality::Diminished) => {
                Ok(SimpleInterval::DiminishedSeventh)
            }
            (SimpleIntervalNumber::Octave, IntervalQuality::Perfect) => {
                Ok(SimpleInterval::PerfectOctave)
            }
            (SimpleIntervalNumber::Octave, IntervalQuality::Major) => {
                Err(InvalidSimpleIntervalError::InvalidMajorNumber)
            }
            (SimpleIntervalNumber::Octave, IntervalQuality::Minor) => {
                Err(InvalidSimpleIntervalError::InvalidMinorNumber)
            }
            (SimpleIntervalNumber::Octave, IntervalQuality::Augmented) => {
                Err(InvalidSimpleIntervalError::InvalidAugmentedNumber)
            }
            (SimpleIntervalNumber::Octave, IntervalQuality::Diminished) => {
                Ok(SimpleInterval::DiminishedOctave)
            }
        }
    }

    /// Given a semitone count, return the interval that represents that
    /// semitone count. If the semitone count is larger than 12 (an octave), the
    /// [`IntervalFromSemitones`] result struct will wrap the interval to the next
    /// octave.
    ///
    /// ```rust
    /// use note_lib::{SimpleInterval, SimpleIntervalFromSemitones};
    ///
    /// let result = SimpleInterval::from_semitones(5);
    /// assert_eq!(result.octave_overflow, 0);
    /// assert_eq!(result.interval, SimpleInterval::PerfectFourth);
    ///
    /// // Inverse of the above, using negative numbers
    /// let result = SimpleInterval::from_semitones(-5);
    /// assert_eq!(result.octave_overflow, -1);
    /// assert_eq!(result.interval, SimpleInterval::PerfectFifth);
    /// assert_eq!(result.interval.inverse(), SimpleInterval::PerfectFourth);
    ///
    /// let result = SimpleInterval::from_semitones(13);
    /// assert_eq!(result.octave_overflow, 1);
    /// assert_eq!(result.interval, SimpleInterval::MinorSecond);
    /// ```
    pub fn from_semitones(semitones: Semitone) -> SimpleIntervalFromSemitones {
        SimpleIntervalFromSemitones::new(semitones)
    }

    /// Get the count of semitones this interval represents.
    pub fn semitones(&self) -> Semitone {
        match self {
            Self::PerfectUnison | Self::DiminishedSecond => 0,
            Self::MinorSecond | Self::AugmentedUnison => 1,
            Self::MajorSecond | Self::DiminishedThird => 2,
            Self::MinorThird | Self::AugmentedSecond => 3,
            Self::MajorThird | Self::DiminishedFourth => 4,
            Self::PerfectFourth | Self::AugmentedThird => 5,
            Self::AugmentedFourth | Self::DiminishedFifth => 6,
            Self::PerfectFifth | Self::DiminishedSixth => 7,
            Self::MinorSixth | Self::AugmentedFifth => 8,
            Self::MajorSixth | Self::DiminishedSeventh => 9,
            Self::MinorSeventh | Self::AugmentedSixth => 10,
            Self::MajorSeventh | Self::DiminishedOctave => 11,
            Self::PerfectOctave | Self::AugmentedSeventh => 12,
        }
    }

    /// Returns the interval number of this interval. For example, a
    /// [`Interval::MinorThird`] has an interval number of [`IntervalNumber::Third`].
    ///
    /// ```rust
    /// use note_lib::{SimpleInterval, SimpleIntervalNumber};
    ///
    /// let result = SimpleInterval::MinorThird.interval_number();
    /// assert_eq!(result, SimpleIntervalNumber::Third);
    /// assert_eq!(result as usize, 3);
    /// ```
    pub fn interval_number(&self) -> SimpleIntervalNumber {
        match self {
            Self::PerfectUnison | Self::AugmentedUnison => SimpleIntervalNumber::Unison,
            Self::DiminishedSecond
            | Self::MinorSecond
            | Self::MajorSecond
            | Self::AugmentedSecond => SimpleIntervalNumber::Second,
            Self::DiminishedThird | Self::MinorThird | Self::MajorThird | Self::AugmentedThird => {
                SimpleIntervalNumber::Third
            }
            Self::DiminishedFourth | Self::PerfectFourth | Self::AugmentedFourth => {
                SimpleIntervalNumber::Fourth
            }
            Self::DiminishedFifth | Self::PerfectFifth | Self::AugmentedFifth => {
                SimpleIntervalNumber::Fifth
            }
            Self::DiminishedSixth | Self::MinorSixth | Self::MajorSixth | Self::AugmentedSixth => {
                SimpleIntervalNumber::Sixth
            }
            Self::DiminishedSeventh
            | Self::MinorSeventh
            | Self::MajorSeventh
            | Self::AugmentedSeventh => SimpleIntervalNumber::Seventh,
            Self::DiminishedOctave | Self::PerfectOctave => SimpleIntervalNumber::Octave,
        }
    }

    /// Returns the interval quality of this interval. For example, a
    /// [`SimpleInterval::MinorThird`] has an interval quality of [`IntervalQuality::Minor`].
    ///
    /// ```rust
    /// use note_lib::{SimpleInterval, IntervalQuality};
    ///
    /// let result = SimpleInterval::MinorThird.quality();
    /// assert_eq!(result, IntervalQuality::Minor);
    /// ```
    pub fn quality(&self) -> IntervalQuality {
        match self {
            Self::PerfectUnison
            | Self::PerfectFourth
            | Self::PerfectFifth
            | Self::PerfectOctave => IntervalQuality::Perfect,
            Self::MinorSecond | Self::MinorThird | Self::MinorSixth | Self::MinorSeventh => {
                IntervalQuality::Minor
            }
            Self::MajorSecond | Self::MajorThird | Self::MajorSixth | Self::MajorSeventh => {
                IntervalQuality::Major
            }
            Self::AugmentedUnison
            | Self::AugmentedSecond
            | Self::AugmentedThird
            | Self::AugmentedFourth
            | Self::AugmentedFifth
            | Self::AugmentedSixth
            | Self::AugmentedSeventh => IntervalQuality::Augmented,
            Self::DiminishedSecond
            | Self::DiminishedThird
            | Self::DiminishedFourth
            | Self::DiminishedFifth
            | Self::DiminishedSixth
            | Self::DiminishedSeventh
            | Self::DiminishedOctave => IntervalQuality::Diminished,
        }
    }

    /// Returns the inverse of this interval. For example, the inverse of a
    /// [`SimpleInterval::MinorThird`] is a [`SimpleInterval::MajorSixth`].
    /// https://en.wikipedia.org/wiki/Interval_(music)#Inversion
    ///
    /// ```rust
    /// use note_lib::SimpleInterval;
    ///
    /// let result = SimpleInterval::MinorThird.inverse();
    /// assert_eq!(result, SimpleInterval::MajorSixth);
    /// assert_eq!(result.inverse(), SimpleInterval::MinorThird);
    /// ```
    pub fn inverse(&self) -> Self {
        // When inverting, this interval and the inverted should add to 9.
        //
        // When inverting, major quality becomes minor, minor becomes major,
        // augmented becomes diminished, diminished becomes augmented, and
        // perfect remains perfect.
        //
        // Ex: m6 + M3 = 9 and M3 + m6 = 9.  M3 is the inverse of m6.

        match self {
            Self::PerfectUnison => Self::PerfectOctave,
            Self::MinorSecond => Self::MajorSeventh,
            Self::MajorSecond => Self::MinorSeventh,
            Self::MinorThird => Self::MajorSixth,
            Self::MajorThird => Self::MinorSixth,
            Self::PerfectFourth => Self::PerfectFifth,
            Self::AugmentedFourth => Self::DiminishedFifth,
            Self::DiminishedFifth => Self::AugmentedFourth,
            Self::PerfectFifth => Self::PerfectFourth,
            Self::MinorSixth => Self::MajorThird,
            Self::MajorSixth => Self::MinorThird,
            Self::MinorSeventh => Self::MajorSecond,
            Self::MajorSeventh => Self::MinorSecond,
            Self::PerfectOctave => Self::PerfectUnison,
            Self::DiminishedSecond => Self::AugmentedSeventh,
            Self::AugmentedUnison => Self::DiminishedOctave,
            Self::DiminishedThird => Self::AugmentedSixth,
            Self::AugmentedSecond => Self::DiminishedSeventh,
            Self::DiminishedFourth => Self::AugmentedFifth,
            Self::AugmentedThird => Self::DiminishedSixth,
            Self::DiminishedSixth => Self::AugmentedThird,
            Self::AugmentedFifth => Self::DiminishedFourth,
            Self::DiminishedSeventh => Self::AugmentedSecond,
            Self::AugmentedSixth => Self::DiminishedThird,
            Self::DiminishedOctave => Self::AugmentedUnison,
            Self::AugmentedSeventh => Self::DiminishedSecond,
        }
    }

    /// Returns an [`IntervalFromSemitones`] result that is the sum of this interval's
    /// semitone representation, and the provided semitones.
    ///
    /// If the semitone sum is larger than 12 (an octave), the [`IntervalFromSemitones`]
    /// result struct will wrap the interval to the next octave.
    ///
    /// Note that the arithmetic will prodiuce perfect, major, and minor intervals
    /// only. Augmented and diminished intervals are coerced to said
    /// intervals during arithmetic.
    ///
    /// ```rust
    /// use note_lib::{SimpleInterval, SimpleIntervalFromSemitones, Semitone};
    ///
    /// let result = SimpleInterval::MajorThird.add_semitones(1);
    /// assert_eq!(result.octave_overflow, 0);
    /// assert_eq!(result.interval, SimpleInterval::PerfectFourth);
    ///
    /// let result = SimpleInterval::MajorThird.add_semitones(12);
    /// assert_eq!(result.octave_overflow, 1);
    /// assert_eq!(result.interval, SimpleInterval::MajorThird);
    ///
    /// let result = SimpleInterval::MajorThird.add_semitones(-2);
    /// assert_eq!(result.octave_overflow, 0);
    /// assert_eq!(result.interval, SimpleInterval::MajorSecond);
    /// ```
    pub fn add_semitones(&self, semitones: Semitone) -> SimpleIntervalFromSemitones {
        SimpleIntervalFromSemitones::new(self.semitones()).add_semitones(semitones)
    }

    /// Given an input interval, will match to an enharmonically equivalent interval
    /// of the given `bias_quality` if one exists. If no enharmonically equivalent
    /// interval exists, or if the input interval is already of the given
    /// `bias_quality`, the input interval is returned.
    pub fn bias_interval_quality(&self, bias_quality: IntervalQuality) -> SimpleInterval {
        // Early return if the quality is already the bias.
        if self.quality() == bias_quality {
            return *self;
        }

        match bias_quality {
            IntervalQuality::Perfect => match self {
                SimpleInterval::DiminishedSecond => SimpleInterval::PerfectUnison,
                SimpleInterval::AugmentedThird => SimpleInterval::PerfectFourth,
                SimpleInterval::DiminishedSixth => SimpleInterval::PerfectFifth,
                SimpleInterval::AugmentedSeventh => SimpleInterval::PerfectOctave,
                _ => *self,
            },
            IntervalQuality::Major => match self {
                SimpleInterval::DiminishedThird => SimpleInterval::MinorSecond,
                SimpleInterval::DiminishedFourth => SimpleInterval::MajorThird,
                SimpleInterval::DiminishedSeventh => SimpleInterval::MajorSixth,
                SimpleInterval::DiminishedOctave => SimpleInterval::MajorSeventh,
                _ => *self,
            },
            IntervalQuality::Minor => match self {
                SimpleInterval::AugmentedUnison => SimpleInterval::MinorSecond,
                SimpleInterval::AugmentedSecond => SimpleInterval::MinorThird,
                SimpleInterval::AugmentedFifth => SimpleInterval::MinorSixth,
                SimpleInterval::AugmentedSixth => SimpleInterval::MinorSeventh,
                _ => *self,
            },
            IntervalQuality::Augmented => match self {
                SimpleInterval::MinorSecond => SimpleInterval::AugmentedUnison,
                SimpleInterval::MinorThird => SimpleInterval::AugmentedSecond,
                SimpleInterval::PerfectFourth => SimpleInterval::AugmentedThird,
                SimpleInterval::MinorSixth => SimpleInterval::AugmentedFifth,
                SimpleInterval::MinorSeventh => SimpleInterval::AugmentedSixth,
                SimpleInterval::PerfectOctave => SimpleInterval::AugmentedSeventh,
                _ => *self,
            },
            IntervalQuality::Diminished => match self {
                SimpleInterval::PerfectUnison => SimpleInterval::DiminishedSecond,
                SimpleInterval::MajorSecond => SimpleInterval::DiminishedThird,
                SimpleInterval::MajorThird => SimpleInterval::DiminishedFourth,
                SimpleInterval::PerfectFifth => SimpleInterval::DiminishedSixth,
                SimpleInterval::MajorSixth => SimpleInterval::DiminishedSeventh,
                SimpleInterval::MajorSeventh => SimpleInterval::DiminishedOctave,
                _ => *self,
            },
        }
    }
}

impl Add<Semitone> for SimpleInterval {
    type Output = SimpleInterval;

    fn add(self, rhs: Semitone) -> Self::Output {
        self.add_semitones(rhs)
            .interval
            .bias_interval_quality(self.quality())
    }
}

impl Add<SimpleInterval> for SimpleInterval {
    type Output = SimpleInterval;

    fn add(self, rhs: SimpleInterval) -> Self::Output {
        self.add_semitones(rhs.semitones())
            .interval
            .bias_interval_quality(self.quality())
    }
}

impl Sub<Semitone> for SimpleInterval {
    type Output = SimpleInterval;

    fn sub(self, rhs: Semitone) -> Self::Output {
        self.add_semitones(-rhs)
            .interval
            .bias_interval_quality(self.quality())
    }
}

impl Sub<SimpleInterval> for SimpleInterval {
    type Output = SimpleInterval;

    fn sub(self, rhs: SimpleInterval) -> Self::Output {
        self.add_semitones(-rhs.semitones())
            .interval
            .bias_interval_quality(self.quality())
    }
}

impl std::fmt::Display for SimpleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if f.alternate() {
            match self {
                SimpleInterval::PerfectUnison => "Perfect Unison",
                SimpleInterval::MinorSecond => "Minor Second",
                SimpleInterval::MajorSecond => "Major Second",
                SimpleInterval::MinorThird => "Minor Third",
                SimpleInterval::MajorThird => "Major Third",
                SimpleInterval::PerfectFourth => "Perfect Fourth",
                SimpleInterval::AugmentedFourth => "Augmented Fourth",
                SimpleInterval::DiminishedFifth => "Diminished Fifth",
                SimpleInterval::PerfectFifth => "Perfect Fifth",
                SimpleInterval::MinorSixth => "Minor Sixth",
                SimpleInterval::MajorSixth => "Major Sixth",
                SimpleInterval::MinorSeventh => "Minor Seventh",
                SimpleInterval::MajorSeventh => "Major Seventh",
                SimpleInterval::PerfectOctave => "Perfect Octave",
                SimpleInterval::DiminishedSecond => "Diminished Second",
                SimpleInterval::AugmentedUnison => "Augmented Unison",
                SimpleInterval::DiminishedThird => "Diminished Third",
                SimpleInterval::AugmentedSecond => "Augmented Second",
                SimpleInterval::DiminishedFourth => "Diminished Fourth",
                SimpleInterval::AugmentedThird => "Augmented Third",
                SimpleInterval::DiminishedSixth => "Diminished Sixth",
                SimpleInterval::AugmentedFifth => "Augmented Fifth",
                SimpleInterval::DiminishedSeventh => "Diminished Seventh",
                SimpleInterval::AugmentedSixth => "Augmented Sixth",
                SimpleInterval::DiminishedOctave => "Diminished Octave",
                SimpleInterval::AugmentedSeventh => "Augmented Seventh",
            }
        } else {
            match self {
                SimpleInterval::PerfectUnison => "PU",
                SimpleInterval::MinorSecond => "m2",
                SimpleInterval::MajorSecond => "M2",
                SimpleInterval::MinorThird => "m3",
                SimpleInterval::MajorThird => "M3",
                SimpleInterval::PerfectFourth => "P4",
                SimpleInterval::AugmentedFourth => "A4",
                SimpleInterval::DiminishedFifth => "d5",
                SimpleInterval::PerfectFifth => "P5",
                SimpleInterval::MinorSixth => "m6",
                SimpleInterval::MajorSixth => "M6",
                SimpleInterval::MinorSeventh => "m7",
                SimpleInterval::MajorSeventh => "M7",
                SimpleInterval::PerfectOctave => "P8",
                SimpleInterval::DiminishedSecond => "d2",
                SimpleInterval::AugmentedUnison => "A1",
                SimpleInterval::DiminishedThird => "d3",
                SimpleInterval::AugmentedSecond => "A2",
                SimpleInterval::DiminishedFourth => "d4",
                SimpleInterval::AugmentedThird => "A3",
                SimpleInterval::DiminishedSixth => "d6",
                SimpleInterval::AugmentedFifth => "A5",
                SimpleInterval::DiminishedSeventh => "d7",
                SimpleInterval::AugmentedSixth => "A6",
                SimpleInterval::DiminishedOctave => "d8",
                SimpleInterval::AugmentedSeventh => "A7",
            }
        };

        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum IntoSimpleIntervalError {
    #[error("No matching simple interval for string: {0}")]
    NoMatchingInterval(String),
}

impl TryFromStringPrefix for SimpleInterval {
    type Error = IntoSimpleIntervalError;

    fn try_from_string_prefix(value: &str) -> Result<(Self, &str), Self::Error> {
        let name = if value.starts_with("PU") {
            SimpleInterval::PerfectUnison
        } else if value.starts_with("m2") {
            SimpleInterval::MinorSecond
        } else if value.starts_with("M2") {
            SimpleInterval::MajorSecond
        } else if value.starts_with("m3") {
            SimpleInterval::MinorThird
        } else if value.starts_with("M3") {
            SimpleInterval::MajorThird
        } else if value.starts_with("P4") {
            SimpleInterval::PerfectFourth
        } else if value.starts_with("A4") {
            SimpleInterval::AugmentedFourth
        } else if value.starts_with("d5") {
            SimpleInterval::DiminishedFifth
        } else if value.starts_with("P5") {
            SimpleInterval::PerfectFifth
        } else if value.starts_with("m6") {
            SimpleInterval::MinorSixth
        } else if value.starts_with("M6") {
            SimpleInterval::MajorSixth
        } else if value.starts_with("m7") {
            SimpleInterval::MinorSeventh
        } else if value.starts_with("M7") {
            SimpleInterval::MajorSeventh
        } else if value.starts_with("P8") {
            SimpleInterval::PerfectOctave
        } else if value.starts_with("d2") {
            SimpleInterval::DiminishedSecond
        } else if value.starts_with("A1") {
            SimpleInterval::AugmentedUnison
        } else if value.starts_with("d3") {
            SimpleInterval::DiminishedThird
        } else if value.starts_with("A2") {
            SimpleInterval::AugmentedSecond
        } else if value.starts_with("d4") {
            SimpleInterval::DiminishedFourth
        } else if value.starts_with("A3") {
            SimpleInterval::AugmentedThird
        } else if value.starts_with("d6") {
            SimpleInterval::DiminishedSixth
        } else if value.starts_with("A5") {
            SimpleInterval::AugmentedFifth
        } else if value.starts_with("d7") {
            SimpleInterval::DiminishedSeventh
        } else if value.starts_with("A6") {
            SimpleInterval::AugmentedSixth
        } else if value.starts_with("d8") {
            SimpleInterval::DiminishedOctave
        } else if value.starts_with("A7") {
            SimpleInterval::AugmentedSeventh
        } else {
            return Err(IntoSimpleIntervalError::NoMatchingInterval(
                value.to_string(),
            ));
        };

        // This is hacky. We could be more efficient
        // by keeping track of the length in this method
        // instead of recomputing it here.
        let prefix_length = name.to_string().len();
        Ok((name, &value[prefix_length..]))
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn from_semitones() {
        // Assert that we can go from an interval to semitones, and back again.
        let minor_second_semitones = SimpleInterval::MinorSecond.semitones();
        assert_eq!(minor_second_semitones, 1);
        let minor_second_result = SimpleInterval::from_semitones(minor_second_semitones);
        assert_eq!(minor_second_result.interval, SimpleInterval::MinorSecond);
        assert_eq!(minor_second_result.octave_overflow, 0);

        // Assert that octaves do not cause "octave_overflow" in conversion
        // from semitones to interval.
        let octave_semitones = SimpleInterval::PerfectOctave.semitones();
        assert_eq!(octave_semitones, 12);
        let octave_result = SimpleInterval::from_semitones(octave_semitones);
        assert_eq!(octave_result.interval, SimpleInterval::PerfectOctave);
        assert_eq!(octave_result.octave_overflow, 0);

        // Assert that if we go past an octave, we correctly overflow and
        // compute the correct interval in the overflowed interval range.
        let octave_plus_minor_second_semitones =
            SimpleInterval::PerfectOctave.semitones() + SimpleInterval::MinorSecond.semitones();
        assert_eq!(octave_plus_minor_second_semitones, 13);
        let octave_plus_minor_second_result =
            SimpleInterval::from_semitones(octave_plus_minor_second_semitones);
        assert_eq!(
            octave_plus_minor_second_result.interval,
            SimpleInterval::MinorSecond
        );
        assert_eq!(octave_plus_minor_second_result.octave_overflow, 1);
    }

    #[test]
    fn inverses_are_correct() {
        // Just asserting the rules I found at
        // https://en.wikipedia.org/wiki/Interval_(music)#Inversion

        SimpleInterval::iter().for_each(|interval| {
            let interval_quality = interval.quality();
            let interval_number = interval.interval_number();

            let inverted = interval.inverse();
            let inverted_quality = inverted.quality();
            let inverted_number = inverted.interval_number();

            assert_eq!(interval, inverted.inverse());
            assert_eq!(interval_number as usize + inverted_number as usize, 9);

            match interval_quality {
                IntervalQuality::Perfect => {
                    assert_eq!(inverted_quality, IntervalQuality::Perfect)
                }
                IntervalQuality::Major => {
                    assert_eq!(inverted_quality, IntervalQuality::Minor)
                }
                IntervalQuality::Minor => {
                    assert_eq!(inverted_quality, IntervalQuality::Major)
                }
                IntervalQuality::Augmented => {
                    assert_eq!(inverted_quality, IntervalQuality::Diminished)
                }
                IntervalQuality::Diminished => {
                    assert_eq!(inverted_quality, IntervalQuality::Augmented)
                }
            }
        });
    }

    #[test]
    fn bias_interval_to_enharmonic_equivalent() {
        // Test that we can get the correct enharmonic equivalent of an interval

        let input = SimpleInterval::PerfectUnison;
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Diminished),
            SimpleInterval::DiminishedSecond
        );
        // There is no enharmonic equivalent of a perfect unison with a major quality.
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Major),
            SimpleInterval::PerfectUnison
        );
        // There is no enharmonic equivalent of a perfect unison with a minor quality.
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Minor),
            SimpleInterval::PerfectUnison
        );
        // There is no enharmoic equivalent of a perfect unison with an augmented quality.
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Augmented),
            SimpleInterval::PerfectUnison
        );
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Perfect),
            SimpleInterval::PerfectUnison
        );

        let input = SimpleInterval::PerfectFourth;
        // There is no enharmonic equivalent of a perfect fourth with a diminished quality.
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Diminished),
            SimpleInterval::PerfectFourth
        );
        // There is no enharmonic equivalent of a perfect fourth with a major quality.
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Major),
            SimpleInterval::PerfectFourth
        );
        // There is no enharmonic equivalent of a perfect fourth with a minor quality.
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Minor),
            SimpleInterval::PerfectFourth
        );
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Augmented),
            SimpleInterval::AugmentedThird
        );
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Perfect),
            SimpleInterval::PerfectFourth
        );

        let input = SimpleInterval::MajorThird;
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Diminished),
            SimpleInterval::DiminishedFourth
        );
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Major),
            SimpleInterval::MajorThird
        );
        // There is no enharmonic equivalent of a major third with a minor quality.
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Minor),
            SimpleInterval::MajorThird
        );
        // There is no enharmonic equivalent of a major third with an augmented quality.
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Augmented),
            SimpleInterval::MajorThird
        );
        // There is no enharmonic equivalent of a major third with a perfect quality.
        assert_eq!(
            input.bias_interval_quality(IntervalQuality::Perfect),
            SimpleInterval::MajorThird
        );
    }
}
