//! This module contains the code for giving a note, and finding the closest chords
//! that are involved in it.
//! For example, if you give the note C#, it will tell you that C# is the 3rd degree of A major, and the 4th degree of G# minor, etc.

use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use note_lib::{AbstractNote, ScaleDegree, ScaleMode, SimpleInterval};

pub fn explore_root(note: AbstractNote) {
    // For now, we will explore ionian and dorian modes.
    // Also, we can pick any octave to start with, since we are only
    // concerned with the abstract note and not the specific note.

    let start_note = note.at_octave(4);

    let mut explored_combos = HashSet::new();

    for mode in ScaleMode::iter_scale_modes() {
        for interval in SimpleInterval::iter_simple_intervals() {
            let exploratory_root = start_note + interval;

            for enharmonic_note in exploratory_root
                .get_enharmonics()
                .chain(once(exploratory_root))
            {
                if explored_combos.contains(&(enharmonic_note.abstract_note(), mode)) {
                    continue;
                }

                explored_combos.insert((enharmonic_note.abstract_note(), mode));

                for degree in ScaleDegree::iter_degrees().filter(|&d| d != ScaleDegree::Octave) {
                    let chord_at_degree = mode.chord_at_degree(enharmonic_note, degree);

                    if let Some(idx) = chord_at_degree.contains_abstract_note(&note) {
                        println!(
                            "{} is exactly in the {} degree of {} {} | {}",
                            note,
                            degree,
                            enharmonic_note.abstract_note(),
                            mode,
                            chord_at_degree
                                .notes()
                                .iter()
                                .map(|n| n.abstract_note())
                                .map(|n| format!("{} ", n))
                                .collect::<String>()
                        );
                    } else if let Some(idx) =
                        chord_at_degree.contains_enharmonic_abstract_note(&note)
                    {
                        println!(
                            "{} is enharmonically in the {} degree of {} {} | {}",
                            note,
                            degree,
                            enharmonic_note.abstract_note(),
                            mode,
                            chord_at_degree
                                .notes()
                                .iter()
                                .map(|n| n.abstract_note())
                                .map(|n| format!("{} ", n))
                                .collect::<String>()
                        );
                    }
                }
            }
        }
    }
}
