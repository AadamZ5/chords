use std::fmt::Display;

use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum_macros::EnumIter)]
pub enum ScaleDegree {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave,
}

impl Display for ScaleDegree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let degree_str = match self {
            ScaleDegree::First => "1st",
            ScaleDegree::Second => "2nd",
            ScaleDegree::Third => "3rd",
            ScaleDegree::Fourth => "4th",
            ScaleDegree::Fifth => "5th",
            ScaleDegree::Sixth => "6th",
            ScaleDegree::Seventh => "7th",
            ScaleDegree::Octave => "8ve",
        };
        write!(f, "{}", degree_str)
    }
}

impl ScaleDegree {
    pub fn iter_degrees() -> ScaleDegreeIter {
        ScaleDegree::iter()
    }
}
