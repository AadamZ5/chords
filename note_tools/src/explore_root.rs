//! This module contains the code for giving a note, and finding the closest chords
//! that are involved in it.
//! For example, if you give the note C#, it will tell you that C# is the 3rd degree of A major, and the 4th degree of G# minor, etc.

use std::iter::once;

use note_lib::{AbstractNote, ScaleDegree, ScaleMode, SimpleInterval};

pub fn explore_root(note: AbstractNote) {
    // For now, we will explore ionian and dorian modes.
    // Also, we can pick any octave to start with, since we are only
    // concerned with the abstract note and not the specific note.

    let start_note = note.at_octave(4);

    for mode in ScaleMode::iter_scale_modes() {
        for interval in SimpleInterval::iter_simple_intervals() {
            let exploratory_root = start_note + interval;

            for enharmonic_note in exploratory_root
                .get_enharmonics()
                .chain(once(exploratory_root))
            {
                println!("Exploring {} {}:", enharmonic_note.abstract_note(), mode);

                for degree in ScaleDegree::iter_degrees().filter(|&d| d != ScaleDegree::Octave) {
                    let chord_at_degree = mode.chord_at_degree(enharmonic_note, degree);

                    if let Some(idx) = chord_at_degree.contains_abstract_note(&note) {
                        println!(
                            "{} is exactly in a {} chord (idx {}: {}), at the {} degree of {} {} | {:?}",
                            note,
                            chord_at_degree.notes()[0].abstract_note(),
                            idx,
                            chord_at_degree.notes()[idx].abstract_note(),
                            degree,
                            exploratory_root.abstract_note(),
                            mode,
                            chord_at_degree
                        );
                    } else if let Some(idx) =
                        chord_at_degree.contains_enharmonic_abstract_note(&note)
                    {
                        println!(
                            "{} is enharmonically in a {} chord (idx {}: {}), at the {} degree of {} {} | {:?}",
                            note,
                            chord_at_degree.notes()[0].abstract_note(),
                            idx,
                            chord_at_degree.notes()[idx].abstract_note(),
                            degree,
                            exploratory_root.abstract_note(),
                            mode,
                            chord_at_degree
                        );
                    }
                }
            }
        }
    }
}
