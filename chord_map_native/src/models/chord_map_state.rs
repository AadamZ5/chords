use eframe::egui::Rect;

use super::chord_view_context::ChordViewContext;

#[derive(Debug)]
pub struct ChordMapState {
    pub delete_chord: Option<ChordViewContext>,
    pub chord_views: Vec<ChordViewContext>,

    /// The rectangle representing the visible area of the chord map scene
    ///
    /// This gets updated as the user pans or zooms in the scene.
    pub map_rect: Rect,
}

impl Default for ChordMapState {
    fn default() -> Self {
        Self {
            delete_chord: None,
            chord_views: Vec::new(),
            map_rect: Rect::from_center_size((0.0, 0.0).into(), (1000.0, 1000.0).into()),
        }
    }
}
