use super::Chord;
use crate::{Interval, Note};

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

fn major_intervals() -> Vec<Interval> {
    vec![
        Interval::PerfectUnison,
        Interval::MajorThird,
        Interval::PerfectFifth,
    ]
}

fn major_6th_intervals() -> Vec<Interval> {
    let mut intervals = major_intervals();
    intervals.push(Interval::MajorSixth);
    intervals
}

fn major_7th_intervals() -> Vec<Interval> {
    let mut intervals = major_intervals();
    intervals.push(Interval::MajorSeventh);
    intervals
}

impl ChordQuality {
    pub fn to_intervals(&self) -> Vec<Interval> {
        todo!()
    }

    pub fn to_notes(&self, root: Note) -> Vec<Note> {
        match self {
            ChordQuality::Major => {
                let third = root.add_semitones(4);
                let fifth = root.add_semitones(7);
                vec![root, third, fifth]
            }
            ChordQuality::Major6th => {
                let third = root.add_semitones(4);
                let fifth = root.add_semitones(7);
                let sixth = root.add_semitones(9);
                vec![root, third, fifth, sixth]
            }
            ChordQuality::Major7th => {
                let third = root.add_semitones(4);
                let fifth = root.add_semitones(7);
                let seventh = root.add_semitones(11);
                vec![root, third, fifth, seventh]
            }
            ChordQuality::Major9th => {
                let third = root.add_semitones(4);
                let fifth = root.add_semitones(7);
                let seventh = root.add_semitones(11);
                let ninth = root.add_semitones(14);
                vec![root, third, fifth, seventh, ninth]
            }
            ChordQuality::Major11th => {
                let third = root.add_semitones(4);
                let fifth = root.add_semitones(7);
                let seventh = root.add_semitones(11);
                let ninth = root.add_semitones(14);
                let eleventh = root.add_semitones(17);
                vec![root, third, fifth, seventh, ninth, eleventh]
            }
            ChordQuality::Major13th => {
                let third = root.add_semitones(4);
                let fifth = root.add_semitones(7);
                let seventh = root.add_semitones(11);
                let ninth = root.add_semitones(14);
                let eleventh = root.add_semitones(17);
                let thirteenth = root.add_semitones(21);
                vec![root, third, fifth, seventh, ninth, eleventh, thirteenth]
            }
            ChordQuality::Minor => {
                let third = root.add_semitones(3);
                let fifth = root.add_semitones(7);
                vec![root, third, fifth]
            }
            ChordQuality::Minor6th => {
                let third = root.add_semitones(3);
                let fifth = root.add_semitones(7);
                let sixth = root.add_semitones(9);
                vec![root, third, fifth, sixth]
            }
            ChordQuality::Minor7th => {
                let third = root.add_semitones(3);
                let fifth = root.add_semitones(7);
                let seventh = root.add_semitones(10);
                vec![root, third, fifth, seventh]
            }
            ChordQuality::MinorMajor7th => {
                let third = root.add_semitones(3);
                let fifth = root.add_semitones(7);
                let seventh = root.add_semitones(11);
                vec![root, third, fifth, seventh]
            }
            ChordQuality::Minor9th => {
                let third = root.add_semitones(3);
                let fifth = root.add_semitones(7);
                let seventh = root.add_semitones(10);
                let ninth = root.add_semitones(14);
                vec![root, third, fifth, seventh, ninth]
            }
            ChordQuality::Minor11th => {
                let third = root.add_semitones(3);
                let fifth = root.add_semitones(7);
                let seventh = root.add_semitones(10);
                let ninth = root.add_semitones(14);
                let eleventh = root.add_semitones(17);
                vec![root, third, fifth, seventh, ninth, eleventh]
            }
            ChordQuality::Minor13th => {
                let third = root.add_semitones(3);
                let fifth = root.add_semitones(7);
                let seventh = root.add_semitones(10);
                let ninth = root.add_semitones(14);
                let eleventh = root.add_semitones(17);
                let thirteenth = root.add_semitones(21);
                vec![root, third, fifth, seventh, ninth, eleventh, thirteenth]
            }
            ChordQuality::MinorMajor7thFlat13th => {
                let third = root.add_semitones(3);
                let fifth = root.add_semitones(7);
                let seventh = root.add_semitones(11);
                let thirteenth = root.add_semitones(20);
                vec![root, third, fifth, seventh, thirteenth]
            }
            ChordQuality::Augmented => {
                let third = root.add_semitones(4);
                let fifth = root.add_semitones(8);
                vec![root, third, fifth]
            }
            ChordQuality::Augmented7th => {
                let third = root.add_semitones(4);
                let fifth = root.add_semitones(8);
                let seventh = root.add_semitones(10);
                vec![root, third, fifth, seventh]
            }
            ChordQuality::AugmentedMajor7th => {
                let third = root.add_semitones(4);
                let fifth = root.add_semitones(8);
                let seventh = root.add_semitones(11);
                vec![root, third, fifth, seventh]
            }
            ChordQuality::Diminished => {
                let third = root.add_semitones(3);
                let fifth = root.add_semitones(6);
                vec![root, third, fifth]
            }
            ChordQuality::Diminished7th => {
                let third = root.add_semitones(3);
                let fifth = root.add_semitones(6);
                let seventh = root.add_semitones(9);
                vec![root, third, fifth, seventh]
            }
            ChordQuality::Suspended2nd => {
                let second = root.add_semitones(2);
                let fifth = root.add_semitones(7);
                vec![root, second, fifth]
            }
            ChordQuality::Suspended4th => {
                let fourth = root.add_semitones(5);
                let fifth = root.add_semitones(7);
                vec![root, fourth, fifth]
            }
        }
    }

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
