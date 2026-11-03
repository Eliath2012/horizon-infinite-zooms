// Remove the old 5% minimum turn rate so extreme zoom can aim precisely.
fn infinite_zoom_turn_floor(old: f32) -> f32 {
    let _ = old;
    0.000001
}
