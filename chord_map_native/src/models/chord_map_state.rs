use super::chord_view_context::ChordViewContext;

#[derive(Debug, Default)]
pub struct ChordMapState {
    pub delete_chord: Option<ChordViewContext>,
    pub chord_views: Vec<ChordViewContext>,

    /// Map X offset from center
    pub map_x: f64,
    /// Map Y offset from center
    pub map_y: f64,
}
