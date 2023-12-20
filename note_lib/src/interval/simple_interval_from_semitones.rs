use crate::{Semitone, SimpleInterval};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimpleIntervalFromSemitones {
    /// How many times the semitones overflowed an entire
    /// octave.
    ///
    /// #[test]
    /// ```rust
    /// use note_lib::{SimpleInterval, Semitone};
    ///
    /// let result = SimpleInterval::from_semitones(13);
    /// assert_eq!(result.octave_overflow, 1);
    /// assert_eq!(result.interval, SimpleInterval::MinorSecond);
    ///
    /// let result = SimpleInterval::from_semitones(12);
    /// assert_eq!(result.octave_overflow, 0);
    /// assert_eq!(result.interval, SimpleInterval::PerfectOctave);
    ///
    /// let result = SimpleInterval::from_semitones(-2);
    /// assert_eq!(result.octave_overflow, -1);
    /// assert_eq!(result.interval, SimpleInterval::MinorSeventh);
    ///
    /// ```
    pub octave_overflow: i32,
    pub interval: SimpleInterval,
}

impl SimpleIntervalFromSemitones {
    pub fn new(semitones: Semitone) -> Self {
        if semitones == 0 {
            return SimpleIntervalFromSemitones {
                interval: SimpleInterval::PerfectUnison,
                octave_overflow: 0,
            };
        }

        if semitones == 12 {
            return SimpleIntervalFromSemitones {
                interval: SimpleInterval::PerfectOctave,
                octave_overflow: 0,
            };
        }

        let (mut octaves, mut remaining_semitones) = (semitones / 12, (semitones % 12));

        if remaining_semitones < 0 {
            remaining_semitones += 12;
            octaves -= 1;
        }

        let interval = match remaining_semitones {
            0 => SimpleInterval::PerfectUnison,
            1 => SimpleInterval::MinorSecond,
            2 => SimpleInterval::MajorSecond,
            3 => SimpleInterval::MinorThird,
            4 => SimpleInterval::MajorThird,
            5 => SimpleInterval::PerfectFourth,
            6 => SimpleInterval::DiminishedFifth,
            7 => SimpleInterval::PerfectFifth,
            8 => SimpleInterval::MinorSixth,
            9 => SimpleInterval::MajorSixth,
            10 => SimpleInterval::MinorSeventh,
            11 => SimpleInterval::MajorSeventh,
            12 => SimpleInterval::PerfectOctave,
            _ => panic!(
                "Semitone overflow after performing modulo 12 on {}",
                semitones
            ),
        };

        SimpleIntervalFromSemitones {
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
