use crate::{
    try_from_string_prefix::TryFromStringPrefix,
    CompoundInterval, IntervalQuality, IntoCompoundIntervalError, Semitone,
    SimpleInterval::{self, PerfectOctave},
};
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Interval {
    /// An interval that fits within one octave.
    Simple(SimpleInterval),
    /// An interval that spans multiple octaves.
    Compound(CompoundInterval),
}

impl Interval {
    pub fn from_semitones(semitones: Semitone) -> Interval {
        if semitones < 13 {
            Interval::Simple(SimpleInterval::from_semitones(semitones).interval)
        } else {
            Interval::Compound(CompoundInterval::from_semitones(semitones))
        }
    }

    pub fn semitones(&self) -> Semitone {
        match self {
            Interval::Simple(simple_interval) => simple_interval.semitones(),
            Interval::Compound(compound_interval) => compound_interval.semitones(),
        }
    }

    pub fn quality(&self) -> IntervalQuality {
        match self {
            Interval::Simple(simple_interval) => simple_interval.quality(),
            Interval::Compound(compound_interval) => compound_interval.quality(),
        }
    }

    pub fn bias_interval_quality(&self, bias_quality: IntervalQuality) -> Interval {
        // Note: There are a few edge cases where a CompoundInterval::DiminishedNinth could be
        // biased to a SimpleInterval::PerfectOctave. We have to handle those special conversions
        // here.

        match bias_quality {
            IntervalQuality::Perfect => match self {
                Self::Compound(CompoundInterval::DiminishedNinth) => {
                    return Self::Simple(PerfectOctave)
                }
                _ => {}
            },
            IntervalQuality::Diminished => match self {
                Self::Simple(PerfectOctave) => {
                    return Self::Compound(CompoundInterval::DiminishedNinth)
                }
                _ => {}
            },
            _ => {}
        };

        match self {
            Interval::Simple(simple_interval) => {
                Interval::Simple(simple_interval.bias_interval_quality(bias_quality))
            }
            Interval::Compound(compound_interval) => {
                Interval::Compound(compound_interval.bias_interval_quality(bias_quality))
            }
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if f.alternate() {
            match self {
                Interval::Simple(simple_interval) => format!("Simple {:#}", simple_interval),
                Interval::Compound(compound_interval) => {
                    format!("Compound {:#}", compound_interval)
                }
            }
        } else {
            match self {
                Interval::Simple(simple_interval) => simple_interval.to_string(),
                Interval::Compound(compound_interval) => compound_interval.to_string(),
            }
        };

        write!(f, "{}", name)
    }
}

impl From<SimpleInterval> for Interval {
    fn from(simple_interval: SimpleInterval) -> Self {
        Interval::Simple(simple_interval)
    }
}

impl From<CompoundInterval> for Interval {
    fn from(compound_interval: CompoundInterval) -> Self {
        Interval::Compound(compound_interval)
    }
}

impl From<Semitone> for Interval {
    fn from(semitones: Semitone) -> Self {
        Interval::from_semitones(semitones)
    }
}

impl Add<Interval> for Interval {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        Interval::from_semitones(self.semitones() + rhs.semitones())
            .bias_interval_quality(self.quality())

        // // Try to add diatonic number and quality first, and if that fails, we can just add semitones.
        // let diatonic_number_sum = self.diatonic_number() + rhs.diatonic_number();
        // let
    }
}

impl Sub<Interval> for Interval {
    type Output = Interval;

    fn sub(self, rhs: Interval) -> Self::Output {
        Interval::from_semitones(self.semitones() - rhs.semitones())
            .bias_interval_quality(self.quality())
    }
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum IntoIntervalError {
    #[error("Invalid interval string: {0}")]
    InvalidIntervalString(String),
    #[error("Unknown interval quality: {0}")]
    InvalidQualityString(String),
    #[error("Invalid combination of diatonic number and quality: {1}{0}")]
    InvalidDiatonicAndQualityCombination(u8, IntervalQuality),
}

impl TryFrom<&str> for Interval {
    type Error = IntoIntervalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (interval, remaining) = Interval::try_from_string_prefix(value)?;
        if !remaining.is_empty() {
            return Err(IntoIntervalError::InvalidIntervalString(value.to_string()));
        }
        Ok(interval)
    }
}

impl TryFromStringPrefix for Interval {
    type Error = IntoIntervalError;

    fn try_from_string_prefix(value: &str) -> Result<(Self, &str), Self::Error> {
        match CompoundInterval::try_from_string_prefix(value) {
            Ok((CompoundInterval::Other(other_interval), remaining_after_compound)) => {
                if let Ok(simple_interval) = SimpleInterval::try_from(&other_interval) {
                    // If the compound interval can be converted to a simple interval, we should prefer
                    // that simpler representation instead of returning a compound interval that is
                    // functionally equivalent to a simple interval.
                    return Ok((Interval::Simple(simple_interval), remaining_after_compound));
                }

                Ok((
                    Interval::Compound(CompoundInterval::Other(other_interval)),
                    remaining_after_compound,
                ))
            }
            Ok((compound_interval, remaining_after_compound)) => Ok((
                Interval::Compound(compound_interval),
                remaining_after_compound,
            )),
            Err(e) => Err(match e {
                IntoCompoundIntervalError::InvalidCompoundIntervalString(v) => {
                    IntoIntervalError::InvalidIntervalString(v)
                }
                IntoCompoundIntervalError::InvalidQuality(q) => {
                    IntoIntervalError::InvalidQualityString(q)
                }
                IntoCompoundIntervalError::InvalidDiatonicNumber(_) => {
                    IntoIntervalError::InvalidIntervalString(value.to_string())
                }
                IntoCompoundIntervalError::InvalidDiatonicAndQualityCombo(n, interval_quality) => {
                    IntoIntervalError::InvalidDiatonicAndQualityCombination(n, interval_quality)
                }
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CompoundInterval, Interval, IntervalQuality, SimpleInterval};

    #[test]
    fn test_interval_quality_biasing() {
        let interval = Interval::Compound(CompoundInterval::DiminishedNinth);
        let biased_interval = interval.bias_interval_quality(IntervalQuality::Perfect);
        assert_eq!(
            biased_interval,
            Interval::Simple(SimpleInterval::PerfectOctave)
        );

        let interval = Interval::Simple(SimpleInterval::PerfectOctave);
        let biased_interval = interval.bias_interval_quality(IntervalQuality::Diminished);
        assert_eq!(
            biased_interval,
            Interval::Compound(CompoundInterval::DiminishedNinth)
        );
    }

    #[test]
    fn test_interval_addition() {
        let interval1 = Interval::Simple(SimpleInterval::MajorThird);
        let interval2 = Interval::Simple(SimpleInterval::PerfectFifth);
        let result = interval1 + interval2;
        assert_eq!(result, Interval::Simple(SimpleInterval::MajorSeventh));

        let interval1 = Interval::Simple(SimpleInterval::MajorThird);
        let interval2 = Interval::Simple(SimpleInterval::MajorThird);
        let result = interval1 + interval2;
        assert_eq!(result, Interval::Simple(SimpleInterval::MinorSixth));

        let interval1 = Interval::Simple(SimpleInterval::PerfectOctave);
        let interval2 = Interval::Simple(SimpleInterval::MajorThird);
        let result = interval1 + interval2;
        assert_eq!(result, Interval::Compound(CompoundInterval::MajorTenth));

        let interval1 = Interval::Simple(SimpleInterval::PerfectOctave);
        let interval2 = Interval::Simple(SimpleInterval::PerfectUnison);
        let result = interval1 + interval2;
        assert_eq!(result, Interval::Simple(SimpleInterval::PerfectOctave));

        let interval1 = Interval::Compound(CompoundInterval::MajorTenth);
        let interval2 = Interval::Compound(CompoundInterval::MajorTenth);

        // A major 10th plus a major 10th should give us a minor 20th, which can be decomposed
        // into two octaves and a minor sixth.
        let result = interval1 + interval2;
        assert!(matches!(
            result,
            Interval::Compound(CompoundInterval::Other(_))
        ));
        if let Interval::Compound(CompoundInterval::Other(other_compound_interval)) = result {
            assert_eq!(other_compound_interval.diatonic_number(), 20);
            assert_eq!(other_compound_interval.quality(), IntervalQuality::Minor);
        } else {
            panic!(
                "Expected an OtherCompoundInterval, got {:#?} instead",
                result
            );
        }
    }
}
