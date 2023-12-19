use chord_map_egui::widgets::chord_view;
use eframe::{
    egui::{Area, Context, Frame},
    emath::Align2,
};

use crate::models::chord_view_context::ChordViewContext;

pub fn chord_map_item(ctx: &Context, chord_ctx: &mut ChordViewContext) {
    let chord_id = chord_ctx.id().to_string() + "_display";
    let area_id: eframe::egui::Id = chord_id.clone().into();

    let area_response = Area::new(area_id)
        .pivot(Align2::CENTER_CENTER)
        .movable(true)
        .current_pos(chord_ctx.map_pos)
        .show(ctx, |ui| {
            let ui_style = ui.style().as_ref();

            let frame = Frame::none()
                .fill(ui_style.visuals.window_fill)
                .stroke(ui_style.visuals.window_stroke)
                .shadow(ui_style.visuals.popup_shadow)
                .inner_margin(ui_style.spacing.window_margin)
                .rounding(ui_style.visuals.window_rounding);

            frame.show(ui, |ui| {
                chord_view(ui, &mut chord_ctx.chord_context);

                let button_text = if chord_ctx.window_open { "🗙" } else { "✏" };

                if ui.button(button_text).clicked() {
                    chord_ctx.window_open = !chord_ctx.window_open;
                }
            });
        });

    let dragged_delta = area_response.response.drag_delta();

    chord_ctx.map_pos += dragged_delta;
}
