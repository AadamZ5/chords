use chord_map_egui::widgets::ChordModifier;
use eframe::{
    egui::{Area, Context, Window},
    epaint::Pos2,
    Frame,
};
use note_lib::models::{Chord, Note, RawNote};

use crate::models::{chord_ctx::ChordCtx, chord_map_state::ChordMapState};

pub fn main_ui(ctx: &Context, app_context: &mut ChordMapState) {
    let ChordMapState {
        ref mut chord_ctxs, ..
    } = app_context;

    eframe::egui::CentralPanel::default().show(ctx, |ui| {
        let clicked = ui.button("Add Chord").clicked();
        if clicked {
            let mut new_ctx = ChordCtx::new(Chord::default());
            new_ctx.window_open = true;
            chord_ctxs.push(new_ctx);
        }
    });
}

pub fn chords_edit_windows(ctx: &Context, app_context: &mut ChordMapState) {
    let ChordMapState {
        ref mut chord_ctxs, ..
    } = app_context;

    for chord_ctx in chord_ctxs.iter_mut() {
        let chord_id = chord_ctx.id().to_string();
        let window_id: eframe::egui::Id = chord_id.clone().into();
        let open_ctx = &mut chord_ctx.window_open;

        Window::new("Chord")
            .id(window_id)
            .open(open_ctx)
            .auto_sized()
            .movable(true)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading(chord_id);
                ui.add(ChordModifier::new(&mut chord_ctx.chord));
            });
    }
}

pub fn chords_display(ctx: &Context, app_context: &mut ChordMapState) {
    let ChordMapState {
        ref mut chord_ctxs, ..
    } = app_context;

    for chord_ctx in chord_ctxs.iter_mut() {
        let chord_id = chord_ctx.id().to_string() + "_display";
        let area_id: eframe::egui::Id = chord_id.clone().into();

        let pos: Pos2 = [chord_ctx.map_x as f32, chord_ctx.map_y as f32].into();

        let area_response = Area::new(area_id)
            .movable(true)
            .current_pos(pos)
            .show(ctx, |ui| {
                ui.heading(chord_id);
                ui.add(ChordModifier::new(&mut chord_ctx.chord));
            });

        let [dx, dy] = area_response.response.drag_delta().into();

        chord_ctx.map_x += dx as f64;
        chord_ctx.map_y += dy as f64;
    }
}
