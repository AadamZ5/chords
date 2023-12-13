use chord_map_egui::widgets::{ChordEdit, ChordView};
use eframe::{
    egui::{Area, Context, Window},
    epaint::Pos2,
    Frame,
};
use note_lib::models::{Chord, ChordQuality, Note, RawNote, C};

use crate::models::{chord_map_state::ChordMapState, chord_view_context::ChordViewContext};

pub fn main_ui(ctx: &Context, app_context: &mut ChordMapState) {
    let ChordMapState {
        chord_views: ref mut chord_ctxs,
        ..
    } = app_context;

    eframe::egui::CentralPanel::default().show(ctx, |ui| {
        let clicked = ui.button("Add Chord").clicked();
        if clicked {
            let mut new_ctx = ChordViewContext::new(Note::new(C, 4), ChordQuality::Major);
            new_ctx.window_open = true;
            chord_ctxs.push(new_ctx);
        }
    });
}

pub fn chords_edit_windows(ctx: &Context, app_context: &mut ChordMapState) {
    let ChordMapState {
        ref mut chord_views,
        ..
    } = app_context;

    for chord_view in chord_views.iter_mut() {
        let chord_id = chord_view.id().to_string();
        let window_id: eframe::egui::Id = chord_id.clone().into();
        let open_ctx = &mut chord_view.window_open;

        Window::new("Chord")
            .id(window_id)
            .open(open_ctx)
            .auto_sized()
            .movable(true)
            .resizable(true)
            .show(ctx, |ui| {
                ui.add(ChordEdit::new(&mut chord_view.chord_context));
            });
    }
}

pub fn chords_display(ctx: &Context, app_context: &mut ChordMapState) {
    let ChordMapState {
        chord_views: ref mut chord_ctxs,
        ..
    } = app_context;

    for chord_ctx in chord_ctxs.iter_mut() {
        let chord_id = chord_ctx.id().to_string() + "_display";
        let area_id: eframe::egui::Id = chord_id.clone().into();

        let pos: Pos2 = [chord_ctx.map_x as f32, chord_ctx.map_y as f32].into();

        let area_response = Area::new(area_id)
            .movable(true)
            .current_pos(pos)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.add(ChordView::new(&mut chord_ctx.chord_context));

                    let button_text = if chord_ctx.window_open {
                        "Close"
                    } else {
                        "Edit"
                    };

                    if ui.button(button_text).clicked() {
                        chord_ctx.window_open = !chord_ctx.window_open;
                    }
                })
            });

        let [dx, dy] = area_response.response.drag_delta().into();

        chord_ctx.map_x += dx as f64;
        chord_ctx.map_y += dy as f64;
    }
}
