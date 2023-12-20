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
use std::fmt::Display;

use crate::{
    bias_simple_interval_quality, IntervalQuality, Semitone, SimpleInterval,
    SimpleIntervalFromSemitones,
};

/// Represents an unusual combination of simple intervals.
#[derive(Debug, Clone, PartialEq, Default, Eq, PartialOrd, Ord)]
pub struct OtherCompoundInterval {
    interval_stack: Vec<SimpleInterval>,
}

impl OtherCompoundInterval {
    pub fn new(mut interval_stack: Vec<SimpleInterval>) -> Self {
        // Sort smallest to largest.
        interval_stack.sort();
        // Put largest in front.
        interval_stack.reverse();
        OtherCompoundInterval { interval_stack }
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
    /// in the stack. If you need the simplified top interval, use
    /// [`OtherCompoundInterval::simple_interval`] instead.
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
    pub fn simple_interval(&self) -> SimpleInterval {
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
        bias_simple_interval_quality(computed_simple_interval, last_interval.quality())
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

    DiminishedTweltfth,
    AuthmentedEleventh,

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
            18 => CompoundInterval::DiminishedTweltfth,
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
            CompoundInterval::DiminishedTweltfth => 18,
            CompoundInterval::AuthmentedEleventh => 18,
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
            CompoundInterval::DiminishedTweltfth => SimpleInterval::DiminishedFifth,
            CompoundInterval::AuthmentedEleventh => SimpleInterval::AugmentedFourth,
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
            CompoundInterval::Other(other) => other.interval_stack.last().unwrap().clone(),
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
                CompoundInterval::DiminishedTweltfth => "Diminished Twelfth",
                CompoundInterval::AuthmentedEleventh => "Augmented Eleventh",
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
                CompoundInterval::DiminishedTweltfth => "d12",
                CompoundInterval::AuthmentedEleventh => "A11",
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
