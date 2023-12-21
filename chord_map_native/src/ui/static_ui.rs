use chord_map_egui::widgets::chord_edit;
use eframe::{
    egui::{Area, Context, Id, Window},
    epaint::Pos2,
};
use note_lib::{Chord, ChordQuality, Note, NoteModifier, RawNote, C};

use crate::models::{chord_map_state::ChordMapState, chord_view_context::ChordViewContext};

use super::app_widgets::{chord_edit_window::chord_edit_window, chord_map_item::chord_map_item};

pub fn main_ui(ctx: &Context, app_context: &mut ChordMapState) {
    let ChordMapState {
        chord_views: ref mut chord_ctxs,
        ..
    } = app_context;

    eframe::egui::CentralPanel::default().show(ctx, |ui| {
        let clicked = ui.button("Add Chord").clicked();
        if clicked {
            let mut new_ctx =
                ChordViewContext::new(Note::new(C, 4, NoteModifier::Natural), ChordQuality::Major);
            new_ctx.window_open = true;
            new_ctx.set_position(ctx.screen_rect().center());
            chord_ctxs.push(new_ctx);
        }
    });
}

pub fn chords_edit_windows(ctx: &Context, app_context: &mut ChordMapState) {
    let ChordMapState {
        ref mut chord_views,
        ..
    } = app_context;

    for chord_view in chord_views.iter_mut().filter(|ctx| ctx.window_open) {
        chord_edit_window(ctx, chord_view)
    }
}

pub fn chords_display(ctx: &Context, app_context: &mut ChordMapState) {
    let ChordMapState {
        ref mut chord_views,
        ..
    } = app_context;

    for chord_ctx in chord_views.iter_mut().filter(|ctx| !ctx.window_open) {
        chord_map_item(ctx, chord_ctx);
    }
}
