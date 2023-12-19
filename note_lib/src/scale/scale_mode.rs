use crate::AbstractNote;

use super::{Interval, ScaleDegree};

/// ScaleMode represents the various patterns of notes that can be created
/// from a root note.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub enum ScaleMode {
    /// Ionian represents the diatonic major scale.
    /// https://en.wikipedia.org/wiki/Mode_(music)#Ionian_(I)
    ///
    /// Interval pattern from root:
    /// P1 | M2 | M3 | P4 | P5 | M6 | M7 | P8
    #[default]
    Ionian,
    /// Dorian is very close to the (Aeolian) natural minor scale, except the
    /// sixth note is major. https://en.wikipedia.org/wiki/Mode_(music)#Dorian_(II)
    ///
    /// Interval pattern from root:
    /// P1 | M2 | m3 | P4 | P5 | M6 | m7 | P8
    Dorian,
    /// Phrygian is similar to the natural minor scale, except the second and sixth
    /// are minor. https://en.wikipedia.org/wiki/Mode_(music)#Phrygian_(III)
    ///
    /// Interval pattern from root:
    /// P1 | m2 | m3 | P4 | P5 | m6 | m7 | P8
    Phrygian,
    /// Lydian is similar to the (Ionian) major scale, except the fourth is augmented.
    /// https://en.wikipedia.org/wiki/Mode_(music)#Lydian_(IV)
    ///
    /// Interval pattern from root:
    /// P1 | M2 | M3 | A4 | P5 | M6 | M7 | P8
    Lydian,
    /// Mixolydian is similar to the (Ionian) major scale, except the seventh is minor.
    /// https://en.wikipedia.org/wiki/Mode_(music)#Mixolydian_(V)
    ///
    /// Interval pattern from root:
    /// P1 | M2 | M3 | P4 | P5 | M6 | m7 | P8
    Mixolydian,
    /// Aeolian is commonly reffered to as the natural minor scale. It is similar to the
    /// (Dorian) only the sixth is minor.
    /// https://en.wikipedia.org/wiki/Mode_(music)#Aeolian_(VI)
    ///
    /// Interval pattern from root:
    /// P1 | M2 | m3 | P4 | P5 | m6 | m7 | P8
    Aeolian,
    /// Loriacn is similar to the (Phrygian) only the fifth is diminished.
    /// https://en.wikipedia.org/wiki/Mode_(music)#Locrian_(VII)
    ///
    /// Interval pattern from root:
    /// P1 | m2 | m3 | P4 | d5 | m6 | m7 | P8
    Locrian,
}

fn ionian_intervals(degree: ScaleDegree) -> Interval {
    match degree {
        ScaleDegree::First => Interval::PerfectUnison,
        ScaleDegree::Second => Interval::MajorSecond,
        ScaleDegree::Third => Interval::MajorThird,
        ScaleDegree::Fourth => Interval::PerfectFourth,
        ScaleDegree::Fifth => Interval::PerfectFifth,
        ScaleDegree::Sixth => Interval::MajorSixth,
        ScaleDegree::Seventh => Interval::MajorSeventh,
        ScaleDegree::Octave => Interval::PerfectOctave,
    }
}

fn dorian_intervals(degree: ScaleDegree) -> Interval {
    match degree {
        ScaleDegree::First => Interval::PerfectUnison,
        ScaleDegree::Second => Interval::MajorSecond,
        ScaleDegree::Third => Interval::MinorThird,
        ScaleDegree::Fourth => Interval::PerfectFourth,
        ScaleDegree::Fifth => Interval::PerfectFifth,
        ScaleDegree::Sixth => Interval::MajorSixth,
        ScaleDegree::Seventh => Interval::MinorSeventh,
        ScaleDegree::Octave => Interval::PerfectOctave,
    }
}

fn phrygian_intervals(degree: ScaleDegree) -> Interval {
    match degree {
        ScaleDegree::First => Interval::PerfectUnison,
        ScaleDegree::Second => Interval::MinorSecond,
        ScaleDegree::Third => Interval::MinorThird,
        ScaleDegree::Fourth => Interval::PerfectFourth,
        ScaleDegree::Fifth => Interval::PerfectFifth,
        ScaleDegree::Sixth => Interval::MinorSixth,
        ScaleDegree::Seventh => Interval::MinorSeventh,
        ScaleDegree::Octave => Interval::PerfectOctave,
    }
}

fn lydian_intervals(degree: ScaleDegree) -> Interval {
    match degree {
        ScaleDegree::First => Interval::PerfectUnison,
        ScaleDegree::Second => Interval::MajorSecond,
        ScaleDegree::Third => Interval::MajorThird,
        ScaleDegree::Fourth => Interval::AugmentedFourth,
        ScaleDegree::Fifth => Interval::PerfectFifth,
        ScaleDegree::Sixth => Interval::MajorSixth,
        ScaleDegree::Seventh => Interval::MajorSeventh,
        ScaleDegree::Octave => Interval::PerfectOctave,
    }
}

fn mixolydian_intervals(degree: ScaleDegree) -> Interval {
    match degree {
        ScaleDegree::First => Interval::PerfectUnison,
        ScaleDegree::Second => Interval::MajorSecond,
        ScaleDegree::Third => Interval::MajorThird,
        ScaleDegree::Fourth => Interval::PerfectFourth,
        ScaleDegree::Fifth => Interval::PerfectFifth,
        ScaleDegree::Sixth => Interval::MajorSixth,
        ScaleDegree::Seventh => Interval::MinorSeventh,
        ScaleDegree::Octave => Interval::PerfectOctave,
    }
}

fn aeolian_intervals(degree: ScaleDegree) -> Interval {
    match degree {
        ScaleDegree::First => Interval::PerfectUnison,
        ScaleDegree::Second => Interval::MajorSecond,
        ScaleDegree::Third => Interval::MinorThird,
        ScaleDegree::Fourth => Interval::PerfectFourth,
        ScaleDegree::Fifth => Interval::PerfectFifth,
        ScaleDegree::Sixth => Interval::MinorSixth,
        ScaleDegree::Seventh => Interval::MinorSeventh,
        ScaleDegree::Octave => Interval::PerfectOctave,
    }
}

fn locrian_intervals(degree: ScaleDegree) -> Interval {
    match degree {
        ScaleDegree::First => Interval::PerfectUnison,
        ScaleDegree::Second => Interval::MinorSecond,
        ScaleDegree::Third => Interval::MinorThird,
        ScaleDegree::Fourth => Interval::PerfectFourth,
        ScaleDegree::Fifth => Interval::DiminishedFifth,
        ScaleDegree::Sixth => Interval::MinorSixth,
        ScaleDegree::Seventh => Interval::MinorSeventh,
        ScaleDegree::Octave => Interval::PerfectOctave,
    }
}

impl ScaleMode {
    /// Get the interval of the degree of the scale.
    ///
    /// In [`ScaleMode::Ionian`] mode, the [`ScaleDegree::Seventh`] is a [`Interval::MajorSeventh`]. In [`ScaleMode::Aeolian`] mode, the
    /// [`ScaleDegree::Seventh`] is a [`Interval::MinorSeventh`]. You can find the corresponding interval
    /// at a degree using this funciton.
    ///
    /// ```rust
    /// use note_lib::{ScaleDegree, ScaleMode, Interval};
    ///
    /// let mode = ScaleMode::Ionian;
    ///
    /// let interval_at_three = mode.interval_at_degree(ScaleDegree::Third);
    /// assert_eq!(interval_at_three, Interval::MajorThird);
    /// ```
    ///
    pub fn interval_at_degree(&self, degree: ScaleDegree) -> Interval {
        match self {
            ScaleMode::Ionian => ionian_intervals(degree),
            ScaleMode::Dorian => dorian_intervals(degree),
            ScaleMode::Phrygian => phrygian_intervals(degree),
            ScaleMode::Lydian => lydian_intervals(degree),
            ScaleMode::Mixolydian => mixolydian_intervals(degree),
            ScaleMode::Aeolian => aeolian_intervals(degree),
            ScaleMode::Locrian => locrian_intervals(degree),
        }
    }

    /// Gets the abstract note at the given degree, using a root note as reference.
    ///
    /// ```rust
    /// use note_lib::{ScaleDegree, ScaleMode, AbstractNote};
    ///
    /// let mode = ScaleMode::Ionian;
    /// let root = AbstractNote::try_from("C").unwrap();
    ///
    /// let note_at_degree = mode.note_at_degree(root, ScaleDegree::Third);
    ///
    /// assert_eq!(note_at_degree, AbstractNote::try_from("E").unwrap());
    /// ```
    pub fn note_at_degree(&self, root: AbstractNote, degree: ScaleDegree) -> AbstractNote {
        let interval = self.interval_at_degree(degree);
        root.add_interval(interval)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mode_gives_interval_at_degree() {
        let mode = ScaleMode::Ionian;
        assert_eq!(
            mode.interval_at_degree(ScaleDegree::First),
            Interval::PerfectUnison
        );
        assert_eq!(
            mode.interval_at_degree(ScaleDegree::Seventh),
            Interval::MajorSeventh
        );
    }

    #[test]
    fn mode_gives_note_at_degree() {
        let mode = ScaleMode::Ionian;
        let root = AbstractNote::try_from("C").unwrap();
        assert_eq!(
            mode.note_at_degree(root, ScaleDegree::First),
            AbstractNote::try_from("C").unwrap()
        );
        assert_eq!(
            mode.note_at_degree(root, ScaleDegree::Seventh),
            AbstractNote::try_from("B").unwrap()
        );
        assert_eq!(
            mode.note_at_degree(root, ScaleDegree::Octave),
            AbstractNote::try_from("C").unwrap()
        );

        let mode = ScaleMode::Ionian;
        let root = AbstractNote::try_from("B#").unwrap();
        assert_eq!(
            mode.note_at_degree(root, ScaleDegree::First),
            AbstractNote::try_from("B#").unwrap()
        );
        assert_eq!(
            mode.note_at_degree(root, ScaleDegree::Seventh),
            AbstractNote::try_from("B").unwrap()
        );
    }

    #[test]
    fn assert_ionian_intervals() {
        assert_eq!(
            ionian_intervals(ScaleDegree::First),
            Interval::PerfectUnison
        );
        assert_eq!(ionian_intervals(ScaleDegree::Second), Interval::MajorSecond);
        assert_eq!(ionian_intervals(ScaleDegree::Third), Interval::MajorThird);
        assert_eq!(
            ionian_intervals(ScaleDegree::Fourth),
            Interval::PerfectFourth
        );
        assert_eq!(ionian_intervals(ScaleDegree::Fifth), Interval::PerfectFifth);
        assert_eq!(ionian_intervals(ScaleDegree::Sixth), Interval::MajorSixth);
        assert_eq!(
            ionian_intervals(ScaleDegree::Seventh),
            Interval::MajorSeventh
        );
        assert_eq!(
            ionian_intervals(ScaleDegree::Octave),
            Interval::PerfectOctave
        );
    }

    #[test]
    fn assert_dorian_intervals() {
        assert_eq!(
            dorian_intervals(ScaleDegree::First),
            Interval::PerfectUnison
        );
        assert_eq!(dorian_intervals(ScaleDegree::Second), Interval::MajorSecond);
        assert_eq!(dorian_intervals(ScaleDegree::Third), Interval::MinorThird);
        assert_eq!(
            dorian_intervals(ScaleDegree::Fourth),
            Interval::PerfectFourth
        );
        assert_eq!(dorian_intervals(ScaleDegree::Fifth), Interval::PerfectFifth);
        assert_eq!(dorian_intervals(ScaleDegree::Sixth), Interval::MajorSixth);
        assert_eq!(
            dorian_intervals(ScaleDegree::Seventh),
            Interval::MinorSeventh
        );
        assert_eq!(
            dorian_intervals(ScaleDegree::Octave),
            Interval::PerfectOctave
        );
    }

    #[test]
    fn assert_phrygian_intervals() {
        assert_eq!(
            phrygian_intervals(ScaleDegree::First),
            Interval::PerfectUnison
        );
        assert_eq!(
            phrygian_intervals(ScaleDegree::Second),
            Interval::MinorSecond
        );
        assert_eq!(phrygian_intervals(ScaleDegree::Third), Interval::MinorThird);
        assert_eq!(
            phrygian_intervals(ScaleDegree::Fourth),
            Interval::PerfectFourth
        );
        assert_eq!(
            phrygian_intervals(ScaleDegree::Fifth),
            Interval::PerfectFifth
        );
        assert_eq!(phrygian_intervals(ScaleDegree::Sixth), Interval::MinorSixth);
        assert_eq!(
            phrygian_intervals(ScaleDegree::Seventh),
            Interval::MinorSeventh
        );
        assert_eq!(
            phrygian_intervals(ScaleDegree::Octave),
            Interval::PerfectOctave
        );
    }

    #[test]
    fn assert_lydian_intervals() {
        assert_eq!(
            lydian_intervals(ScaleDegree::First),
            Interval::PerfectUnison
        );
        assert_eq!(lydian_intervals(ScaleDegree::Second), Interval::MajorSecond);
        assert_eq!(lydian_intervals(ScaleDegree::Third), Interval::MajorThird);
        assert_eq!(
            lydian_intervals(ScaleDegree::Fourth),
            Interval::AugmentedFourth
        );
        assert_eq!(lydian_intervals(ScaleDegree::Fifth), Interval::PerfectFifth);
        assert_eq!(lydian_intervals(ScaleDegree::Sixth), Interval::MajorSixth);
        assert_eq!(
            lydian_intervals(ScaleDegree::Seventh),
            Interval::MajorSeventh
        );
        assert_eq!(
            lydian_intervals(ScaleDegree::Octave),
            Interval::PerfectOctave
        );
    }

    #[test]
    fn assert_mixolydian_intervals() {
        assert_eq!(
            mixolydian_intervals(ScaleDegree::First),
            Interval::PerfectUnison
        );
        assert_eq!(
            mixolydian_intervals(ScaleDegree::Second),
            Interval::MajorSecond
        );
        assert_eq!(
            mixolydian_intervals(ScaleDegree::Third),
            Interval::MajorThird
        );
        assert_eq!(
            mixolydian_intervals(ScaleDegree::Fourth),
            Interval::PerfectFourth
        );
        assert_eq!(
            mixolydian_intervals(ScaleDegree::Fifth),
            Interval::PerfectFifth
        );
        assert_eq!(
            mixolydian_intervals(ScaleDegree::Sixth),
            Interval::MajorSixth
        );
        assert_eq!(
            mixolydian_intervals(ScaleDegree::Seventh),
            Interval::MinorSeventh
        );
        assert_eq!(
            mixolydian_intervals(ScaleDegree::Octave),
            Interval::PerfectOctave
        );
    }

    #[test]
    fn assert_aeolian_intervals() {
        assert_eq!(
            aeolian_intervals(ScaleDegree::First),
            Interval::PerfectUnison
        );
        assert_eq!(
            aeolian_intervals(ScaleDegree::Second),
            Interval::MajorSecond
        );
        assert_eq!(aeolian_intervals(ScaleDegree::Third), Interval::MinorThird);
        assert_eq!(
            aeolian_intervals(ScaleDegree::Fourth),
            Interval::PerfectFourth
        );
        assert_eq!(
            aeolian_intervals(ScaleDegree::Fifth),
            Interval::PerfectFifth
        );
        assert_eq!(aeolian_intervals(ScaleDegree::Sixth), Interval::MinorSixth);
        assert_eq!(
            aeolian_intervals(ScaleDegree::Seventh),
            Interval::MinorSeventh
        );
        assert_eq!(
            aeolian_intervals(ScaleDegree::Octave),
            Interval::PerfectOctave
        );
    }

    #[test]
    fn assert_locrian_intervals() {
        assert_eq!(
            locrian_intervals(ScaleDegree::First),
            Interval::PerfectUnison
        );
        assert_eq!(
            locrian_intervals(ScaleDegree::Second),
            Interval::MinorSecond
        );
        assert_eq!(locrian_intervals(ScaleDegree::Third), Interval::MinorThird);
        assert_eq!(
            locrian_intervals(ScaleDegree::Fourth),
            Interval::PerfectFourth
        );
        assert_eq!(
            locrian_intervals(ScaleDegree::Fifth),
            Interval::DiminishedFifth
        );
        assert_eq!(locrian_intervals(ScaleDegree::Sixth), Interval::MinorSixth);
        assert_eq!(
            locrian_intervals(ScaleDegree::Seventh),
            Interval::MinorSeventh
        );
        assert_eq!(
            locrian_intervals(ScaleDegree::Octave),
            Interval::PerfectOctave
        );
    }
}
