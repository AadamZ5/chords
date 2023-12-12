use chord_map_egui::models::{Note, RawNote};

fn main() {
    let note = Note::new(RawNote::A, 4);
    let note2 = Note::new(RawNote::B, 4);

    let chord = note + note2;

    println!("{:#}, {}", note, note.to_hertz());
    println!("{:#}, {}", note2, note2.to_hertz());
    println!("{:#?}", chord);
}
