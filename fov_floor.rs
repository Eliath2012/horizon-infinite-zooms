// Horizon's fov_degrees checks 1.0..=179.0 after native_compute_fov.
// Allow genuinely narrow positive FOVs for high native zoom, without
// widening anything or disabling finite-value checks.
fn infinite_zoom_minimum_fov(old: f32) -> f32 {
    let _ = old;
    0.00001
}
