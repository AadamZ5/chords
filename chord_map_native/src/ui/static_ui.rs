use eframe::egui::{Rect, Scene, Ui, WidgetText};
use note_lib::{ChordQuality, Note, NoteModifier, C};

use crate::models::{chord_map_state::ChordMapState, chord_view_context::ChordViewContext};

use super::app_widgets::{chord_edit_window::chord_edit_window, chord_map_item::chord_map_item};

pub fn main_ui(ui: &mut Ui, app_context: &mut ChordMapState) {
    top_panel(ui, app_context);
    chord_map(ui, app_context);
}

pub fn top_panel(ui: &mut Ui, app_context: &mut ChordMapState) {
    let ChordMapState {
        chord_views: ref mut chord_ctxs,
        ..
    } = app_context;

    eframe::egui::Panel::top("top_panel").show_inside(ui, |ui| {
        let clicked = ui.button("Add Chord").clicked();
        if clicked {
            let mut new_ctx =
                ChordViewContext::new(Note::new(C, 4, NoteModifier::Natural), ChordQuality::Major);
            new_ctx.window_open = true;
            new_ctx.set_position(ui.content_rect().center());
            chord_ctxs.push(new_ctx);
        }
    });
}

pub fn chord_map(ui: &mut Ui, app_context: &mut ChordMapState) {
    let ChordMapState {
        ref mut map_rect,
        ref mut chord_views,
        ..
    } = app_context;

    let mut scene_content_bounding_rect = Rect::NAN;

    let scene_response = Scene::new()
        .zoom_range(0.1..=2.0)
        .show(ui, map_rect, |ui| {
            ui.heading(format!("Chord Contexts: {}", chord_views.len()));
            chords_display(ui, chord_views);
            chords_edit_windows(ui, chord_views);
            scene_content_bounding_rect = ui.min_rect();
        })
        .response;

    if scene_response.double_clicked() {
        *map_rect = scene_content_bounding_rect;
    }
}

pub fn chords_edit_windows(ui: &mut Ui, chord_views: &mut Vec<ChordViewContext>) {
    for chord_view in chord_views.iter_mut().filter(|ctx| ctx.window_open) {
        chord_edit_window(ui, chord_view)
    }
}

pub fn chords_display(ui: &mut Ui, chord_views: &mut Vec<ChordViewContext>) {
    for chord_ctx in chord_views.iter_mut().filter(|ctx| !ctx.window_open) {
        chord_map_item(ui, chord_ctx);
    }
}
