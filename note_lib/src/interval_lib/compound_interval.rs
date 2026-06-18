//! Compound intervals are larger spanning intervals, like a 9th or 11th. They do not
//! fit within one octave. They are composed of multiple simple intervals, stacked on
//! each other. For example, a 9th is a major 2nd stacked on a perfect 8th. A 11th is
//! a perfect 4th stacked on a perfect 8th.
//!
//! You may notice the obvious concern here with the fact that the numbers do not
//! simply add together. This is because within an octacve, intervals start at 1.
//! The first (1) interval is a [`Interval::PerfectUnison`] which is 0 semitones.
//! This is the source of the "off-by-one" arithmetic when adding/stacking intervals.
//!
//! You wouldn't typically compound an [`Interval::PerfectUnison`] since it represents
//! 0 semitones, but it is arithmetically possible.
use std::{fmt::Display, iter::repeat_n, ops::Add};

use crate::{
    try_from_string_prefix::{try_integer_from_string_prefix, TryFromStringPrefix},
    Interval, IntervalQuality, Semitone, SimpleInterval, SimpleIntervalFromSemitones,
    SimpleIntervalNumber::{self, Octave, Unison},
};

/// Represents an unusual or extended combination of simple intervals.
///
/// This can be arbitrary simple intervals stacked together, or a count of octaves with a simple interval on top.
///
#[derive(Debug, Clone, PartialEq, Default, Eq, PartialOrd, Ord)]
pub struct OtherCompoundInterval {
    interval_stack: Vec<SimpleInterval>,
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum FromDiatonicNumberAndQualityError {
    #[error("Invalid combination of diatonic number and quality: {0}{1}.")]
    InvalidQualityCombination(u8, IntervalQuality),
}

impl OtherCompoundInterval {
    pub fn new<T>(interval_stack: T) -> Self
    where
        T: IntoIterator<Item = SimpleInterval>,
    {
        OtherCompoundInterval {
            interval_stack: interval_stack.into_iter().collect(),
        }
    }

    pub fn new_from_diatonic_number_and_quality(
        diatonic_number: u8,
        quality: IntervalQuality,
    ) -> Result<Self, FromDiatonicNumberAndQualityError> {
        // A diatonic number of 8 is an octave, but a quality of diminished may make it fit within one octave.
        // We need to decompose the diatonic number and interval quality into simple intervals.
        // For example, a A21 (augmented 21th) can be decomposed into two perfect octaves and
        // an A5 (augmented 5th) ( P8 + P8 + A5 = A21)

        // Note that traversing an octave changes the diatonic number by 7, not 8, because of the "off-by-one" nature of interval numbering.
        // Refer to https://en.wikipedia.org/wiki/Interval_(music)#Compound_intervals for more details on this arithmetic.

        // Remove the common note across arithmetic
        let mut diatonic_remaining = diatonic_number;

        let mut simple_intervals = Vec::new();

        while (diatonic_remaining) >= 8 {
            // If there is more than an octave remaining, we should fit in a perfect octave.
            simple_intervals.push(SimpleInterval::PerfectOctave);
            // Subtract the octave minus the common unison
            diatonic_remaining -= Octave as u8 - Unison as u8;
        }

        // At this point, there should be between 0 and 8 diatonic numbers remaining.
        // If we are an augmented octave, that decomposes into a perfect octave and augmented unison.
        if let Ok(simple_interval_number) = SimpleIntervalNumber::try_from(diatonic_remaining) {
            if quality == IntervalQuality::Augmented
                && simple_interval_number == SimpleIntervalNumber::Octave
            {
                // We will decompose into a perfect octave and augmented unison
                simple_intervals.push(SimpleInterval::PerfectOctave);
                simple_intervals.push(SimpleInterval::AugmentedUnison);
            } else if !(quality == IntervalQuality::Perfect
                && simple_interval_number == SimpleIntervalNumber::Unison)
            {
                simple_intervals.push(
                    SimpleInterval::from_quality_and_number(quality, simple_interval_number)
                        .map_err(|_| {
                            FromDiatonicNumberAndQualityError::InvalidQualityCombination(
                                diatonic_number,
                                quality,
                            )
                        })?,
                );
            };
        };

        Ok(OtherCompoundInterval::new(simple_intervals))
    }

    /// Calculates the diatonic number from this compound interval.
    /// Follows the formulat described at
    /// https://en.wikipedia.org/wiki/Interval_(music)#Compound_intervals
    pub fn diatonic_number(&self) -> i32 {
        1 + self.interval_stack.iter().fold(0, |acc, simple_interval| {
            acc + (simple_interval.interval_number() as i32 - 1)
        })
    }

    /// Gets the quality of the top-most simple interval.
    /// This does not simplify the interval to be based on stacked
    /// octaves, it simply returns the quality of the top-most interval
    /// supplied.
    ///
    /// If the simplified quality is needed, use [`OtherCompoundInterval::simple_interval`]
    /// and [`SimpleInterval::quality`] instead.
    pub fn quality(&self) -> IntervalQuality {
        self.top_interval().quality()
    }

    /// Given some compound interval composed of N stacked simple intervals,
    /// return the Nth simple interval. This means the highest is returned.
    ///
    /// This value is not based on underlying octaves, simply the last interval
    /// in the stack. If you need the simplified top interval,
    /// use [`OtherCompoundInterval::top_simple_interval`] instead or alternatively
    /// simplify the entire compound interval using [`OtherCompoundInterval::simplify`]
    /// and then get the top interval of that simplified interval.
    pub fn top_interval(&self) -> SimpleInterval {
        *self
            .interval_stack
            .last()
            .unwrap_or(&SimpleInterval::PerfectUnison)
    }

    /// Given the entire interval range this compound interval spans, return
    /// the simple interval that represents remaining semitones when as many
    /// octaves are fit into this interval.
    ///
    /// This value is simplified from the underlying supplied stack of simple intervals,
    /// meaning if you provide M5, M5, M3, the simplified interval will be the result
    /// of adding the semitones of M5 + M5 + M3, and then taking the simple interval of
    /// that new compound interval.
    ///
    /// For example, if you provide M5, M5, M3, the simplified interval will be d5 or A4.
    /// This is the same as providing a stack of P8 and d5, or a stack of P8 and A4.
    pub fn top_simple_interval(&self) -> SimpleInterval {
        let semitones = self
            .interval_stack
            .iter()
            .fold(0, |acc, simple_interval| acc + simple_interval.semitones());

        if semitones == 0 {
            return SimpleInterval::PerfectUnison;
        }

        let last_interval = self.top_interval();
        let computed_simple_interval = SimpleInterval::from_semitones(semitones).interval;

        // If we got what we actually had, no need to try and adjust the interval quality.
        if last_interval == computed_simple_interval {
            return computed_simple_interval;
        }

        // Try to see if we can align our computed simple interval quality with our top-most interval.
        computed_simple_interval.bias_interval_quality(last_interval.quality())
    }

    /// Simplifies this compound interval to be represented as a stack of octaves and a
    /// final arbitrary simple interval.
    pub fn simplify(&self) -> OtherCompoundInterval {
        let this_quality = self.quality();

        let semitones = self
            .interval_stack
            .iter()
            .fold(0, |acc, simple_interval| acc + simple_interval.semitones());

        if semitones == 0 {
            return OtherCompoundInterval::new(std::iter::empty());
        }

        let simple_interval_from_semitones = SimpleInterval::from_semitones(semitones);

        let octaves = repeat_n(
            SimpleInterval::PerfectOctave,
            simple_interval_from_semitones.octave_overflow as usize,
        );
        let simple_interval = simple_interval_from_semitones
            .interval
            .bias_interval_quality(this_quality);
        OtherCompoundInterval::new(octaves.chain(std::iter::once(simple_interval)))
    }

    /// Given a compound interval, bias the quality of the top-most simple interval to match the provided quality.
    pub fn bias_interval_quality(&self, quality: IntervalQuality) -> OtherCompoundInterval {
        let mut iter = self.interval_stack.iter().copied();
        let last = match iter.next_back() {
            Some(last) => last,
            None => return OtherCompoundInterval::new(iter),
        };

        let mut all_but_last: Vec<_> = iter.collect();

        let all = {
            let biased_last = last.bias_interval_quality(quality);
            all_but_last.push(biased_last);
            all_but_last
        };

        OtherCompoundInterval::new(all)
    }

    /// Returns a view on the stack of simple intervals that compose this compound interval.
    pub fn stack(&self) -> &[SimpleInterval] {
        &self.interval_stack
    }
}

impl Display for OtherCompoundInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let diatonic_number = self.diatonic_number();
        let quality = self.quality();
        write!(f, "{}{}", quality, diatonic_number)
    }
}

impl From<SimpleIntervalFromSemitones> for OtherCompoundInterval {
    fn from(interval_from_semitones: SimpleIntervalFromSemitones) -> Self {
        let octave_span = interval_from_semitones.octave_overflow;
        let mut interval_stack = if octave_span > 0 {
            vec![SimpleInterval::PerfectOctave; octave_span as usize]
        } else {
            vec![]
        };
        interval_stack.push(interval_from_semitones.interval);

        OtherCompoundInterval::new(interval_stack)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompoundInterval {
    // Functionally equivalent to a perfect octave
    DiminishedNinth,

    MinorNinth,
    AugmentedOctave,

    MajorNinth,
    DiminishedTenth,

    MinorTenth,
    AugmentedNinth,

    MajorTenth,
    DiminishedEleventh,

    PerfectEleventh,
    AugmentedTenth,

    DiminishedTwelfth,
    AugmentedEleventh,

    PerfectTwelfth,
    DiminishedThirteenth,

    MinorThirteenth,
    AugmentedTwelfth,

    MajorThirteenth,
    DiminishedFourteenth,

    MinorFourteenth,
    AugmentedThirteenth,

    MajorFourteenth,
    DiminishedFifteenth,

    PerfectFifteenth,
    AugmentedFourteenth,

    AugmentedFifteenth,

    Other(OtherCompoundInterval),
}

impl CompoundInterval {
    pub fn from_semitones(semitones: Semitone) -> CompoundInterval {
        match semitones {
            13 => CompoundInterval::MinorNinth,
            14 => CompoundInterval::MajorNinth,
            15 => CompoundInterval::MinorTenth,
            16 => CompoundInterval::MajorTenth,
            17 => CompoundInterval::PerfectEleventh,
            18 => CompoundInterval::DiminishedTwelfth,
            19 => CompoundInterval::PerfectTwelfth,
            20 => CompoundInterval::MinorThirteenth,
            21 => CompoundInterval::MajorThirteenth,
            22 => CompoundInterval::MinorFourteenth,
            23 => CompoundInterval::MajorFourteenth,
            24 => CompoundInterval::PerfectFifteenth,
            25 => CompoundInterval::AugmentedFifteenth,
            _ => CompoundInterval::Other(SimpleInterval::from_semitones(semitones).into()),
        }
    }

    pub fn semitones(&self) -> Semitone {
        match self {
            CompoundInterval::DiminishedNinth => 13,
            CompoundInterval::MinorNinth => 13,
            CompoundInterval::AugmentedOctave => 13,
            CompoundInterval::MajorNinth => 14,
            CompoundInterval::DiminishedTenth => 14,
            CompoundInterval::MinorTenth => 15,
            CompoundInterval::AugmentedNinth => 15,
            CompoundInterval::MajorTenth => 16,
            CompoundInterval::DiminishedEleventh => 16,
            CompoundInterval::PerfectEleventh => 17,
            CompoundInterval::AugmentedTenth => 17,
            CompoundInterval::DiminishedTwelfth => 18,
            CompoundInterval::AugmentedEleventh => 18,
            CompoundInterval::PerfectTwelfth => 19,
            CompoundInterval::DiminishedThirteenth => 20,
            CompoundInterval::MinorThirteenth => 20,
            CompoundInterval::AugmentedTwelfth => 20,
            CompoundInterval::MajorThirteenth => 21,
            CompoundInterval::DiminishedFourteenth => 22,
            CompoundInterval::MinorFourteenth => 22,
            CompoundInterval::AugmentedThirteenth => 22,
            CompoundInterval::MajorFourteenth => 23,
            CompoundInterval::DiminishedFifteenth => 24,
            CompoundInterval::PerfectFifteenth => 24,
            CompoundInterval::AugmentedFourteenth => 24,
            CompoundInterval::AugmentedFifteenth => 25,
            CompoundInterval::Other(other) => other
                .interval_stack
                .iter()
                .fold(0, |acc, simple_interval| acc + simple_interval.semitones()),
        }
    }

    /// Given a compound interval, return the top-most simple interval.
    /// For example, a compound interval of a 9th would return a simple interval
    /// of a 2nd.
    ///
    pub fn get_simple_interval(&self) -> SimpleInterval {
        match self {
            CompoundInterval::DiminishedNinth => SimpleInterval::PerfectUnison,
            CompoundInterval::MinorNinth => SimpleInterval::MinorSecond,
            CompoundInterval::AugmentedOctave => SimpleInterval::AugmentedUnison,
            CompoundInterval::MajorNinth => SimpleInterval::MajorSecond,
            CompoundInterval::DiminishedTenth => SimpleInterval::DiminishedThird,
            CompoundInterval::MinorTenth => SimpleInterval::MinorThird,
            CompoundInterval::AugmentedNinth => SimpleInterval::AugmentedSecond,
            CompoundInterval::MajorTenth => SimpleInterval::MajorThird,
            CompoundInterval::DiminishedEleventh => SimpleInterval::DiminishedFourth,
            CompoundInterval::PerfectEleventh => SimpleInterval::PerfectFourth,
            CompoundInterval::AugmentedTenth => SimpleInterval::AugmentedThird,
            CompoundInterval::DiminishedTwelfth => SimpleInterval::DiminishedFifth,
            CompoundInterval::AugmentedEleventh => SimpleInterval::AugmentedFourth,
            CompoundInterval::PerfectTwelfth => SimpleInterval::PerfectFifth,
            CompoundInterval::DiminishedThirteenth => SimpleInterval::DiminishedSixth,
            CompoundInterval::MinorThirteenth => SimpleInterval::MinorSixth,
            CompoundInterval::AugmentedTwelfth => SimpleInterval::AugmentedFifth,
            CompoundInterval::MajorThirteenth => SimpleInterval::MajorSixth,
            CompoundInterval::DiminishedFourteenth => SimpleInterval::DiminishedSeventh,
            CompoundInterval::MinorFourteenth => SimpleInterval::MinorSeventh,
            CompoundInterval::AugmentedThirteenth => SimpleInterval::AugmentedSixth,
            CompoundInterval::MajorFourteenth => SimpleInterval::MajorSeventh,
            CompoundInterval::DiminishedFifteenth => SimpleInterval::DiminishedOctave,
            CompoundInterval::PerfectFifteenth => SimpleInterval::PerfectOctave,
            CompoundInterval::AugmentedFourteenth => SimpleInterval::AugmentedSeventh,
            // Augmented 15th is beyond 2 octaves, so the simple interval
            // is the interval within the 3rd octave.
            CompoundInterval::AugmentedFifteenth => SimpleInterval::AugmentedUnison,
            CompoundInterval::Other(other) => *other.interval_stack.last().unwrap(),
        }
    }

    pub fn quality(&self) -> IntervalQuality {
        match self {
            Self::PerfectEleventh | Self::PerfectTwelfth | Self::PerfectFifteenth => {
                IntervalQuality::Perfect
            }
            Self::MajorNinth | Self::MajorTenth | Self::MajorThirteenth | Self::MajorFourteenth => {
                IntervalQuality::Major
            }
            Self::AugmentedOctave
            | Self::AugmentedNinth
            | Self::AugmentedTenth
            | Self::AugmentedEleventh
            | Self::AugmentedTwelfth
            | Self::AugmentedThirteenth
            | Self::AugmentedFourteenth
            | Self::AugmentedFifteenth => IntervalQuality::Augmented,
            Self::MinorNinth | Self::MinorTenth | Self::MinorThirteenth | Self::MinorFourteenth => {
                IntervalQuality::Minor
            }
            Self::DiminishedNinth
            | Self::DiminishedTenth
            | Self::DiminishedEleventh
            | Self::DiminishedTwelfth
            | Self::DiminishedThirteenth
            | Self::DiminishedFourteenth
            | Self::DiminishedFifteenth => IntervalQuality::Diminished,
            Self::Other(other) => other.quality(),
        }
    }

    pub fn bias_interval_quality(&self, quality: IntervalQuality) -> CompoundInterval {
        match self {
            Self::Other(other) => return Self::Other(other.bias_interval_quality(quality)),
            _ => {}
        };

        match quality {
            IntervalQuality::Perfect => match self {
                Self::AugmentedTenth => Self::PerfectEleventh,
                Self::DiminishedThirteenth => Self::PerfectTwelfth,
                Self::AugmentedFourteenth => Self::PerfectFifteenth,
                _ => self.clone(),
            },
            IntervalQuality::Major => match self {
                Self::DiminishedTenth => Self::MajorNinth,
                Self::DiminishedEleventh => Self::MajorTenth,
                Self::DiminishedFourteenth => Self::MajorThirteenth,
                Self::DiminishedFifteenth => Self::MajorFourteenth,
                _ => self.clone(),
            },
            IntervalQuality::Minor => match self {
                Self::AugmentedNinth => Self::MinorTenth,
                Self::AugmentedTwelfth => Self::MinorThirteenth,
                Self::AugmentedThirteenth => Self::MinorFourteenth,
                _ => self.clone(),
            },
            IntervalQuality::Augmented => match self {
                Self::MinorNinth => Self::AugmentedOctave,
                Self::MinorTenth => Self::AugmentedNinth,
                Self::PerfectEleventh => Self::AugmentedTenth,
                Self::MinorThirteenth => Self::AugmentedTwelfth,
                Self::MinorFourteenth => Self::AugmentedThirteenth,
                Self::PerfectFifteenth => Self::AugmentedFourteenth,
                _ => self.clone(),
            },
            IntervalQuality::Diminished => match self {
                Self::MajorNinth => Self::DiminishedTenth,
                Self::MajorTenth => Self::DiminishedEleventh,
                Self::AugmentedEleventh => Self::DiminishedTwelfth,
                Self::PerfectTwelfth => Self::DiminishedThirteenth,
                Self::MajorThirteenth => Self::DiminishedFourteenth,
                Self::MajorFourteenth => Self::DiminishedFifteenth,
                _ => self.clone(),
            },
        }
    }
}

impl Display for CompoundInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if f.alternate() {
            match self {
                CompoundInterval::DiminishedNinth => "Diminished Ninth",
                CompoundInterval::MinorNinth => "Minor Ninth",
                CompoundInterval::AugmentedOctave => "Augmented Octave",
                CompoundInterval::MajorNinth => "Major Ninth",
                CompoundInterval::DiminishedTenth => "Diminished Tenth",
                CompoundInterval::MinorTenth => "Minor Tenth",
                CompoundInterval::AugmentedNinth => "Augmented Ninth",
                CompoundInterval::MajorTenth => "Major Tenth",
                CompoundInterval::DiminishedEleventh => "Diminished Eleventh",
                CompoundInterval::PerfectEleventh => "Perfect Eleventh",
                CompoundInterval::AugmentedTenth => "Augmented Tenth",
                CompoundInterval::DiminishedTwelfth => "Diminished Twelfth",
                CompoundInterval::AugmentedEleventh => "Augmented Eleventh",
                CompoundInterval::PerfectTwelfth => "Perfect Twelfth",
                CompoundInterval::DiminishedThirteenth => "Diminished Thirteenth",
                CompoundInterval::MinorThirteenth => "Minor Thirteenth",
                CompoundInterval::AugmentedTwelfth => "Augmented Twelfth",
                CompoundInterval::MajorThirteenth => "Major Thirteenth",
                CompoundInterval::DiminishedFourteenth => "Diminished Fourteenth",
                CompoundInterval::MinorFourteenth => "Minor Fourteenth",
                CompoundInterval::AugmentedThirteenth => "Augmented Thirteenth",
                CompoundInterval::MajorFourteenth => "Major Fourteenth",
                CompoundInterval::DiminishedFifteenth => "Diminished Fifteenth",
                CompoundInterval::PerfectFifteenth => "Perfect Fifteenth",
                CompoundInterval::AugmentedFourteenth => "Augmented Fourteenth",
                CompoundInterval::AugmentedFifteenth => "Augmented Fifteenth",
                CompoundInterval::Other(other) => return write!(f, "{:#}", other),
            }
        } else {
            match self {
                CompoundInterval::DiminishedNinth => "d9",
                CompoundInterval::MinorNinth => "m9",
                CompoundInterval::AugmentedOctave => "A8",
                CompoundInterval::MajorNinth => "M9",
                CompoundInterval::DiminishedTenth => "d10",
                CompoundInterval::MinorTenth => "m10",
                CompoundInterval::AugmentedNinth => "A9",
                CompoundInterval::MajorTenth => "M10",
                CompoundInterval::DiminishedEleventh => "d11",
                CompoundInterval::PerfectEleventh => "P11",
                CompoundInterval::AugmentedTenth => "A10",
                CompoundInterval::DiminishedTwelfth => "d12",
                CompoundInterval::AugmentedEleventh => "A11",
                CompoundInterval::PerfectTwelfth => "P12",
                CompoundInterval::DiminishedThirteenth => "d13",
                CompoundInterval::MinorThirteenth => "m13",
                CompoundInterval::AugmentedTwelfth => "A12",
                CompoundInterval::MajorThirteenth => "M13",
                CompoundInterval::DiminishedFourteenth => "d14",
                CompoundInterval::MinorFourteenth => "m14",
                CompoundInterval::AugmentedThirteenth => "A13",
                CompoundInterval::MajorFourteenth => "M14",
                CompoundInterval::DiminishedFifteenth => "d15",
                CompoundInterval::PerfectFifteenth => "P15",
                CompoundInterval::AugmentedFourteenth => "A14",
                CompoundInterval::AugmentedFifteenth => "A15",
                CompoundInterval::Other(other) => return write!(f, "{}", other),
            }
        };

        write!(f, "{}", name)
    }
}

impl Add<SimpleInterval> for CompoundInterval {
    type Output = Interval;

    fn add(self, rhs: SimpleInterval) -> Self::Output {
        Interval::from_semitones(self.semitones() + rhs.semitones())
            .bias_interval_quality(self.quality())
    }
}

impl Add<Interval> for CompoundInterval {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        Interval::from_semitones(self.semitones() + rhs.semitones())
            .bias_interval_quality(self.quality())
    }
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum IntoCompoundIntervalError {
    #[error("Invalid compound interval string: {0}")]
    InvalidCompoundIntervalString(String),
    #[error("Invalid quality in compound interval string: {0}")]
    InvalidQuality(String),
    #[error("Invalid diatonic number in compound interval string: {0}")]
    InvalidDiatonicNumber(String),
    #[error(
        "Invalid combination of diatonic number and quality in compound interval string: {1}{0}"
    )]
    InvalidDiatonicAndQualityCombo(u8, IntervalQuality),
}

impl TryFromStringPrefix for CompoundInterval {
    type Error = IntoCompoundIntervalError;

    fn try_from_string_prefix(value: &str) -> Result<(Self, &str), Self::Error> {
        // We can either match against a pre-defined compount interval, or we can try to parse an "other" compound interval
        // based on the expected format of a quality followed by a diatonic number. For example, "M9" would be a major 9th, and "A11" would be an augmented 11th.

        let (compound_interval, remaining) = if value.starts_with("d9") {
            (CompoundInterval::DiminishedNinth, &value[2..])
        } else if value.starts_with("m9") {
            (CompoundInterval::MinorNinth, &value[2..])
        } else if value.starts_with("A8") {
            (CompoundInterval::AugmentedOctave, &value[2..])
        } else if value.starts_with("M9") {
            (CompoundInterval::MajorNinth, &value[2..])
        } else if value.starts_with("d10") {
            (CompoundInterval::DiminishedTenth, &value[3..])
        } else if value.starts_with("m10") {
            (CompoundInterval::MinorTenth, &value[3..])
        } else if value.starts_with("A9") {
            (CompoundInterval::AugmentedNinth, &value[2..])
        } else if value.starts_with("M10") {
            (CompoundInterval::MajorTenth, &value[3..])
        } else if value.starts_with("d11") {
            (CompoundInterval::DiminishedEleventh, &value[3..])
        } else if value.starts_with("P11") {
            (CompoundInterval::PerfectEleventh, &value[3..])
        } else if value.starts_with("A10") {
            (CompoundInterval::AugmentedTenth, &value[3..])
        } else if value.starts_with("d12") {
            (CompoundInterval::DiminishedTwelfth, &value[3..])
        } else if value.starts_with("A11") {
            (CompoundInterval::AugmentedEleventh, &value[3..])
        } else if value.starts_with("P12") {
            (CompoundInterval::PerfectTwelfth, &value[3..])
        } else if value.starts_with("d13") {
            (CompoundInterval::DiminishedThirteenth, &value[3..])
        } else if value.starts_with("m13") {
            (CompoundInterval::MinorThirteenth, &value[3..])
        } else if value.starts_with("A12") {
            (CompoundInterval::AugmentedTwelfth, &value[3..])
        } else if value.starts_with("M13") {
            (CompoundInterval::MajorThirteenth, &value[3..])
        } else if value.starts_with("d14") {
            (CompoundInterval::DiminishedFourteenth, &value[3..])
        } else if value.starts_with("m14") {
            (CompoundInterval::MinorFourteenth, &value[3..])
        } else if value.starts_with("A13") {
            (CompoundInterval::AugmentedThirteenth, &value[3..])
        } else if value.starts_with("M14") {
            (CompoundInterval::MajorFourteenth, &value[3..])
        } else if value.starts_with("d15") {
            (CompoundInterval::DiminishedFifteenth, &value[3..])
        } else if value.starts_with("P15") {
            (CompoundInterval::PerfectFifteenth, &value[3..])
        } else if value.starts_with("A14") {
            (CompoundInterval::AugmentedFourteenth, &value[3..])
        } else if value.starts_with("A15") {
            (CompoundInterval::AugmentedFifteenth, &value[3..])
        } else {
            // Try to parse an "other" compound interval based on the expected format of a quality followed by a diatonic number. For example, "M9" would be a major 9th, and "A11" would be an augmented 11th.
            let (quality, remaining_after_quality) = IntervalQuality::try_from_string_prefix(value)
                .map_err(|_| IntoCompoundIntervalError::InvalidQuality(value.to_string()))?;

            let (diatonic_number, remaining_after_diatonic_num) =
                if let Ok((diatonic_number, remaining)) =
                    try_integer_from_string_prefix(remaining_after_quality)
                {
                    (diatonic_number, remaining)
                } else {
                    return Err(IntoCompoundIntervalError::InvalidDiatonicNumber(
                        value.to_string(),
                    ));
                };

            // Try to form the "other" compound interval
            let other_compound_interval =
                OtherCompoundInterval::new_from_diatonic_number_and_quality(
                    diatonic_number as u8,
                    quality,
                )
                .map_err(|e| match e {
                    FromDiatonicNumberAndQualityError::InvalidQualityCombination(
                        diatonic,
                        quality,
                    ) => {
                        IntoCompoundIntervalError::InvalidDiatonicAndQualityCombo(diatonic, quality)
                    }
                })?;

            (
                CompoundInterval::Other(other_compound_interval),
                remaining_after_diatonic_num,
            )
        };

        Ok((compound_interval, remaining))
    }
}

#[cfg(test)]
mod tests {
    use crate::CompoundInterval;

    use super::*;

    #[test]
    fn test_other_compound_interval_from_diatonic_number_and_quality() {
        let compound_interval =
            OtherCompoundInterval::new_from_diatonic_number_and_quality(9, IntervalQuality::Major)
                .unwrap();
        assert_eq!(compound_interval.diatonic_number(), 9);
        assert_eq!(compound_interval.quality(), IntervalQuality::Major);
        assert_eq!(
            compound_interval.stack(),
            &[SimpleInterval::PerfectOctave, SimpleInterval::MajorSecond]
        );

        let compount_interval = OtherCompoundInterval::new_from_diatonic_number_and_quality(
            11,
            IntervalQuality::Augmented,
        )
        .unwrap();
        assert_eq!(compount_interval.diatonic_number(), 11);
        assert_eq!(compount_interval.quality(), IntervalQuality::Augmented);
        assert_eq!(
            compount_interval.stack(),
            &[
                SimpleInterval::PerfectOctave,
                SimpleInterval::AugmentedFourth
            ]
        );

        let compound_interval =
            OtherCompoundInterval::new_from_diatonic_number_and_quality(17, IntervalQuality::Major)
                .unwrap();
        assert_eq!(compound_interval.diatonic_number(), 17);
        assert_eq!(compound_interval.quality(), IntervalQuality::Major);
        assert_eq!(
            compound_interval.stack(),
            &[
                SimpleInterval::PerfectOctave,
                SimpleInterval::PerfectOctave,
                SimpleInterval::MajorThird
            ]
        );
    }

    #[test]
    fn test_compound_interval_from_string_prefix() {
        let (compound_interval, remaining) =
            CompoundInterval::try_from_string_prefix("M9").unwrap();
        assert_eq!(compound_interval, CompoundInterval::MajorNinth);
        assert_eq!(remaining, "");

        let (compound_interval, remaining) =
            CompoundInterval::try_from_string_prefix("A11").unwrap();
        assert_eq!(compound_interval, CompoundInterval::AugmentedEleventh);
        assert_eq!(remaining, "");

        let (compound_interval, remaining) =
            CompoundInterval::try_from_string_prefix("d16").unwrap();
        assert_eq!(
            compound_interval,
            CompoundInterval::Other(OtherCompoundInterval::new(vec![
                SimpleInterval::PerfectOctave,
                SimpleInterval::PerfectOctave,
                SimpleInterval::DiminishedSecond
            ]))
        );
        assert_eq!(remaining, "");
    }
}
