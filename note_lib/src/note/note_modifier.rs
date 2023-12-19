use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum NoteModifier {
    Sharp,
    Flat,
    #[default]
    Natural,
    DoubleSharp,
    DoubleFlat,
    // TODO: How do we handle microtonal hoopla?
}

impl Display for NoteModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let modifier = match self {
            NoteModifier::Sharp => "#",
            NoteModifier::Flat => "b",
            NoteModifier::Natural => "",
            NoteModifier::DoubleSharp => "##",
            NoteModifier::DoubleFlat => "bb",
        };

        write!(f, "{}", modifier)
    }
}
