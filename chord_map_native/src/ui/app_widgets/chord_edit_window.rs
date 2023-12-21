use chord_map_egui::widgets::chord_edit;
use eframe::{
    egui::{Context, Id, Vec2, Window},
    emath::Align2,
};

use crate::models::chord_view_context::ChordViewContext;

pub const CHORD_EDIT_WINDOW_SIZE: Vec2 = Vec2::new(300.0, 300.0);

pub fn chord_edit_window(ctx: &Context, chord_view: &mut ChordViewContext) {
    let chord_id = chord_view.id().to_string();
    let window_id: Id = Into::<Id>::into(chord_id.clone()).with("edit");

    let actual_chord_context = &mut chord_view.chord_context;
    let editing_chord_context_opt = &mut chord_view.editing_chord_context;
    let editing_chord_context =
        editing_chord_context_opt.get_or_insert_with(|| actual_chord_context.clone());
    let open_ctx = &mut chord_view.window_open;

    let window_response_opt = Window::new(format!("Edit {}", &actual_chord_context))
        .pivot(Align2::CENTER_CENTER)
        .id(window_id)
        .open(open_ctx)
        .fixed_size(CHORD_EDIT_WINDOW_SIZE)
        .current_pos(chord_view.map_pos)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| chord_edit(ui, editing_chord_context));

    // Window response is Some(...) only when it's opened.
    if let Some(window_response) = window_response_opt {
        // We need to manually get the window's area rect by ID since we don't get the real thing back in the response
        // when we constrain the window by using `current_pos(...)`.
        chord_view.map_pos = ctx.memory(|mem| {
            mem.area_rect(window_id)
                .map(|rect| rect.center())
                .unwrap_or(chord_view.map_pos)
        });

        if let Some(edit_action) = window_response.inner.flatten() {
            match edit_action {
                chord_map_egui::widgets::ChordEditAction::Commit => {
                    *actual_chord_context = editing_chord_context.clone();
                    editing_chord_context_opt.take();
                }
                chord_map_egui::widgets::ChordEditAction::Cancel => {
                    editing_chord_context_opt.take();
                }
            }
            *open_ctx = false;
        }
    }
}
