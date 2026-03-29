use std::f32::INFINITY;

use chord_map_egui::widgets::chord_view;
use eframe::{
    egui::{Align, Area, Frame, Layout, Pos2, Rect, Response, Sense, Ui, UiBuilder, Vec2},
    emath::Align2,
};

use crate::models::chord_view_context::ChordViewContext;

pub fn chord_map_item(ui: &mut Ui, chord_ctx: &mut ChordViewContext) {
    ui.heading(format!("{}", chord_ctx.chord_context));

    let chord_id = chord_ctx.id().to_string() + "_display";
    let area_id: eframe::egui::Id = chord_id.clone().into();

    let ui_style = ui.style().as_ref();

    let frame = Frame::new()
        .fill(ui_style.visuals.window_fill)
        .stroke(ui_style.visuals.window_stroke)
        .shadow(ui_style.visuals.popup_shadow)
        .inner_margin(ui_style.spacing.window_margin)
        .corner_radius(ui_style.visuals.window_corner_radius);

    let place_content_rect = Rect::from_min_max(chord_ctx.map_pos, Pos2::new(INFINITY, INFINITY));

    let drag_response = ui
        .new_child(
            UiBuilder::new()
                .layout(Layout::top_down(Align::Min))
                .max_rect(place_content_rect),
        )
        .add(|ui: &mut Ui| {
            let frame_response = frame.show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(format!("Coords: {}", chord_ctx.map_pos));
                    chord_view(ui, &mut chord_ctx.chord_context);

                    let button_text = if chord_ctx.window_open { "🗙" } else { "✏" };

                    if ui.button(button_text).clicked() {
                        chord_ctx.window_open = !chord_ctx.window_open;
                    }
                })
            });

            ui.interact(frame_response.response.rect, area_id, Sense::drag())
        });

    if drag_response.dragged() {
        chord_ctx.map_pos += drag_response.drag_delta();
    }
}
