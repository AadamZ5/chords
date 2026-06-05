use super::Chord;
use crate::{
    CompoundInterval::{self},
    Interval, Note, SimpleInterval,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ChordQuality {
    #[default]
    Major,
    Major6th,
    Major7th,
    Major9th,
    Major11th,
    Major13th,
    Minor,
    Minor6th,
    Minor7th,
    MinorMajor7th,
    Minor9th,
    Minor11th,
    Minor13th,
    MinorMajor7thFlat13th,
    Augmented,
    Augmented7th,
    AugmentedMajor7th,
    Diminished,
    Diminished7th,
    Suspended2nd,
    Suspended4th,
}

impl ChordQuality {
    /// Returns an iterator of intervals to add to the root note
    /// to form the other notes in the chord.
    pub fn to_intervals(&self) -> impl ExactSizeIterator<Item = Interval> {
        self.intervals_slice().iter().cloned()
    }

    /// Returns an iterator of all the notes in the chord, given the root note.
    pub fn to_notes(&self, root: Note) -> impl Iterator<Item = Note> {
        let intervals = self.to_intervals();

        // Build notes by adding intervals to the root note.
        std::iter::once(root).chain(intervals.map(move |interval| root.add_interval(interval)))
    }

    /// Returns a chord with the notes of the quality formed from the root note.
    pub fn to_chord(&self, root: Note) -> Chord {
        Chord::new(self.to_notes(root))
    }

    pub fn short_name(&self) -> &str {
        match self {
            ChordQuality::Major => "maj",
            ChordQuality::Major6th => "maj6",
            ChordQuality::Major7th => "maj7",
            ChordQuality::Major9th => "maj9",
            ChordQuality::Major11th => "maj11",
            ChordQuality::Major13th => "maj13",
            ChordQuality::Minor => "m",
            ChordQuality::Minor6th => "m6",
            ChordQuality::Minor7th => "m7",
            ChordQuality::MinorMajor7th => "mM7",
            ChordQuality::Minor9th => "m9",
            ChordQuality::Minor11th => "m11",
            ChordQuality::Minor13th => "m13",
            ChordQuality::MinorMajor7thFlat13th => "mM7b13",
            ChordQuality::Augmented => "aug",
            ChordQuality::Augmented7th => "aug7",
            ChordQuality::AugmentedMajor7th => "augM7",
            ChordQuality::Diminished => "dim",
            ChordQuality::Diminished7th => "dim7",
            ChordQuality::Suspended2nd => "sus2",
            ChordQuality::Suspended4th => "sus4",
        }
    }

    pub fn long_name(&self) -> &str {
        match self {
            ChordQuality::Major => "Major",
            ChordQuality::Major6th => "Major 6th",
            ChordQuality::Major7th => "Major 7th",
            ChordQuality::Major9th => "Major 9th",
            ChordQuality::Major11th => "Major 11th",
            ChordQuality::Major13th => "Major 13th",
            ChordQuality::Minor => "Minor",
            ChordQuality::Minor6th => "Minor 6th",
            ChordQuality::Minor7th => "Minor 7th",
            ChordQuality::MinorMajor7th => "Minor Major 7th",
            ChordQuality::Minor9th => "Minor 9th",
            ChordQuality::Minor11th => "Minor 11th",
            ChordQuality::Minor13th => "Minor 13th",
            ChordQuality::MinorMajor7thFlat13th => "Minor Major 7th Flat 13th",
            ChordQuality::Augmented => "Augmented",
            ChordQuality::Augmented7th => "Augmented 7th",
            ChordQuality::AugmentedMajor7th => "Augmented Major 7th",
            ChordQuality::Diminished => "Diminished",
            ChordQuality::Diminished7th => "Diminished 7th",
            ChordQuality::Suspended2nd => "Suspended 2nd",
            ChordQuality::Suspended4th => "Suspended 4th",
        }
    }
}

// Private impl
impl ChordQuality {
    /// Returns the intervals to add to the root note to get the other notes in the chord.
    const fn intervals_slice(&self) -> &'static [Interval] {
        use CompoundInterval::*;
        use Interval::*;
        use SimpleInterval::*;

        // I like to refer to this wiki page:
        // https://en.wikipedia.org/wiki/Interval_(music)#Compound_intervals

        match self {
            ChordQuality::Major => &[Simple(MajorThird), Simple(PerfectFifth)],
            ChordQuality::Major6th => {
                &[Simple(MajorThird), Simple(PerfectFifth), Simple(MajorSixth)]
            }
            ChordQuality::Major7th => &[
                Simple(MajorThird),
                Simple(PerfectFifth),
                Simple(MajorSeventh),
            ],
            ChordQuality::Major9th => &[
                Simple(MajorThird),
                Simple(PerfectFifth),
                Simple(MajorSeventh),
                Compound(MajorNinth),
            ],
            ChordQuality::Major11th => &[
                Simple(MajorThird),
                Simple(PerfectFifth),
                Simple(MajorSeventh),
                Compound(MajorNinth),
                Compound(PerfectEleventh),
            ],
            ChordQuality::Major13th => &[
                Simple(MajorThird),
                Simple(PerfectFifth),
                Simple(MajorSeventh),
                Compound(MajorNinth),
                Compound(PerfectEleventh),
                Compound(MajorThirteenth),
            ],
            ChordQuality::Minor => &[Simple(MinorThird), Simple(PerfectFifth)],
            ChordQuality::Minor6th => {
                &[Simple(MinorThird), Simple(PerfectFifth), Simple(MinorSixth)]
            }
            ChordQuality::Minor7th => &[
                Simple(MinorThird),
                Simple(PerfectFifth),
                Simple(MinorSeventh),
            ],
            ChordQuality::MinorMajor7th => &[
                Simple(MinorThird),
                Simple(PerfectFifth),
                Simple(MajorSeventh),
            ],
            ChordQuality::Minor9th => &[
                Simple(MinorThird),
                Simple(PerfectFifth),
                Simple(MinorSeventh),
                Compound(MajorNinth),
            ],
            ChordQuality::Minor11th => &[
                Simple(MinorThird),
                Simple(PerfectFifth),
                Simple(MinorSeventh),
                Compound(MajorNinth),
                Compound(PerfectEleventh),
            ],
            ChordQuality::Minor13th => &[
                Simple(MinorThird),
                Simple(PerfectFifth),
                Simple(MinorSeventh),
                Compound(MajorNinth),
                Compound(PerfectEleventh),
                Compound(MajorThirteenth),
            ],
            ChordQuality::MinorMajor7thFlat13th => &[
                Simple(MinorThird),
                Simple(PerfectFifth),
                Simple(MajorSeventh),
                Compound(MajorThirteenth),
            ],
            ChordQuality::Augmented => &[Simple(MajorThird), Simple(AugmentedFifth)],
            ChordQuality::Augmented7th => &[
                Simple(MajorThird),
                Simple(AugmentedFifth),
                Simple(MinorSeventh),
            ],
            ChordQuality::AugmentedMajor7th => &[
                Simple(MajorThird),
                Simple(AugmentedFifth),
                Simple(MajorSeventh),
            ],
            ChordQuality::Diminished => &[Simple(MinorThird), Simple(DiminishedFifth)],
            ChordQuality::Diminished7th => &[
                Simple(MinorThird),
                Simple(DiminishedFifth),
                Simple(DiminishedSeventh),
            ],
            ChordQuality::Suspended2nd => &[Simple(MajorSecond), Simple(PerfectFifth)],
            ChordQuality::Suspended4th => &[Simple(PerfectFourth), Simple(PerfectFifth)],
        }
    }
}
