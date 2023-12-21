use crate::{CompoundInterval, Semitone, SimpleInterval};
use std::fmt::Display;

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
