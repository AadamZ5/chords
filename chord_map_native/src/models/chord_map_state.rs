use super::chord_ctx::ChordCtx;

#[derive(Debug, Default)]
pub struct ChordMapState {
    pub delete_chord: Option<ChordCtx>,
    pub chord_ctxs: Vec<ChordCtx>,

    /// Map X offset from center
    pub map_x: f64,
    /// Map Y offset from center
    pub map_y: f64,
}
