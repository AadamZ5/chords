use crate::Semitone;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    PerfectUnison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth,
    DiminishedFifth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    PerfectOctave,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntervalFromSemitones {
    /// How many times the semitones overflowed an entire
    /// octave.
    ///
    /// #[test]
    /// ```rust
    /// use note_lib::{Interval, Semitone};
    ///
    /// let result = Interval::from_semitone_interval(13);
    /// assert_eq!(result.octave_overflow, 1);
    /// assert_eq!(result.interval, Interval::MinorSecond);
    ///
    /// let result = Interval::from_semitone_interval(12);
    /// assert_eq!(result.octave_overflow, 0);
    /// assert_eq!(result.interval, Interval::PerfectOctave);
    ///
    /// ```
    pub octave_overflow: i32,
    pub interval: Interval,
}

impl IntervalFromSemitones {
    pub fn new(semitones: Semitone) -> Self {
        if semitones == 0 {
            return IntervalFromSemitones {
                interval: Interval::PerfectUnison,
                octave_overflow: 0,
            };
        }

        if semitones == 12 {
            return IntervalFromSemitones {
                interval: Interval::PerfectOctave,
                octave_overflow: 0,
            };
        }

        let (mut octaves, mut remaining_semitones) = (semitones / 12, (semitones % 12));

        if remaining_semitones < 0 {
            remaining_semitones += 12;
            octaves -= 1;
        }

        let interval = match remaining_semitones {
            0 => Interval::PerfectUnison,
            1 => Interval::MinorSecond,
            2 => Interval::MajorSecond,
            3 => Interval::MinorThird,
            4 => Interval::MajorThird,
            5 => Interval::PerfectFourth,
            6 => Interval::DiminishedFifth,
            7 => Interval::PerfectFifth,
            8 => Interval::MinorSixth,
            9 => Interval::MajorSixth,
            10 => Interval::MinorSeventh,
            11 => Interval::MajorSeventh,
            12 => Interval::PerfectOctave,
            _ => panic!(
                "Semitone overflow after performing modulo 12 on {}",
                semitones
            ),
        };

        IntervalFromSemitones {
            interval,
            octave_overflow: octaves,
        }
    }

    pub fn add_semitones(&self, semitones: Semitone) -> Self {
        let current_octave_overflow = self.octave_overflow;
        let current_interval = self.interval;

        let current_interval_semitones = current_interval.semitones();
        let new_interval_semitones = current_interval_semitones + semitones;

        let mut temp_new_result = Self::new(new_interval_semitones);

        temp_new_result.octave_overflow = current_octave_overflow + temp_new_result.octave_overflow;

        temp_new_result
    }

    pub fn semitones(&self) -> Semitone {
        self.interval.semitones() + (self.octave_overflow as Semitone * 12)
    }
}

impl Interval {
    pub fn semitones(&self) -> Semitone {
        match self {
            Interval::PerfectUnison => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::PerfectFourth => 5,
            Interval::AugmentedFourth => 6,
            Interval::DiminishedFifth => 6,
            Interval::PerfectFifth => 7,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
            Interval::PerfectOctave => 12,
        }
    }

    pub fn from_semitone_interval(semitones: Semitone) -> IntervalFromSemitones {
        IntervalFromSemitones::new(semitones)
    }

    pub fn add_semitones(&self, semitones: Semitone) -> IntervalFromSemitones {
        IntervalFromSemitones::new(self.semitones()).add_semitones(semitones)
    }
}

// TODO: Arithmetic operations on intervals? Add a semitone to an interval? Add two intervals together?

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if f.alternate() {
            match self {
                Interval::PerfectUnison => "Perfect Unison",
                Interval::MinorSecond => "Minor Second",
                Interval::MajorSecond => "Major Second",
                Interval::MinorThird => "Minor Third",
                Interval::MajorThird => "Major Third",
                Interval::PerfectFourth => "Perfect Fourth",
                Interval::AugmentedFourth => "Augmented Fourth",
                Interval::DiminishedFifth => "Diminished Fifth",
                Interval::PerfectFifth => "Perfect Fifth",
                Interval::MinorSixth => "Minor Sixth",
                Interval::MajorSixth => "Major Sixth",
                Interval::MinorSeventh => "Minor Seventh",
                Interval::MajorSeventh => "Major Seventh",
                Interval::PerfectOctave => "Perfect Octave",
            }
        } else {
            match self {
                Interval::PerfectUnison => "PU",
                Interval::MinorSecond => "m2",
                Interval::MajorSecond => "M2",
                Interval::MinorThird => "m3",
                Interval::MajorThird => "M3",
                Interval::PerfectFourth => "P4",
                Interval::AugmentedFourth => "A4",
                Interval::DiminishedFifth => "d5",
                Interval::PerfectFifth => "P5",
                Interval::MinorSixth => "m6",
                Interval::MajorSixth => "M6",
                Interval::MinorSeventh => "m7",
                Interval::MajorSeventh => "M7",
                Interval::PerfectOctave => "P8",
            }
        };

        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_semitones() {
        // Assert that we can go from an interval to semitones, and back again.
        let minor_second_semitones = Interval::MinorSecond.semitones();
        assert_eq!(minor_second_semitones, 1);
        let minor_second_result = Interval::from_semitone_interval(minor_second_semitones);
        assert_eq!(minor_second_result.interval, Interval::MinorSecond);
        assert_eq!(minor_second_result.octave_overflow, 0);

        // Assert that octaves do not cause "octave_overflow" in conversion
        // from semitones to interval.
        let octave_semitones = Interval::PerfectOctave.semitones();
        assert_eq!(octave_semitones, 12);
        let octave_result = Interval::from_semitone_interval(octave_semitones);
        assert_eq!(octave_result.interval, Interval::PerfectOctave);
        assert_eq!(octave_result.octave_overflow, 0);

        // Assert that if we go past an octave, we correctly overflow and
        // compute the correct interval in the overflowed interval range.
        let octave_plus_minor_second_semitones =
            Interval::PerfectOctave.semitones() + Interval::MinorSecond.semitones();
        assert_eq!(octave_plus_minor_second_semitones, 13);
        let octave_plus_minor_second_result =
            Interval::from_semitone_interval(octave_plus_minor_second_semitones);
        assert_eq!(
            octave_plus_minor_second_result.interval,
            Interval::MinorSecond
        );
        assert_eq!(octave_plus_minor_second_result.octave_overflow, 1);
    }
}
