use note_lib::{CompoundInterval, Note, NoteModifier, RawNote};

fn main() {
    let note = Note::new(RawNote::C, 4, NoteModifier::Natural);
    let note2 = Note::new(RawNote::B, 4, NoteModifier::Natural);

    let _chord = note + note2;

    println!("{}, {}", note, note.to_hertz());
    println!("{}, {}", note2, note2.to_hertz());
    //println!("{:#?}", chord);

    let aug_fifteenth = CompoundInterval::from_semitones(25);
    let some_thing = CompoundInterval::from_semitones(32);
    println!("{}", aug_fifteenth);
    println!("{}", some_thing);
}
