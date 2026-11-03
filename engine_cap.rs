// Replace Horizon native_zoom_now's hard 10x cap. Change only that constant.
fn infinite_zoom_engine_cap(old: f32) -> f32 {
    let _ = old;
    crate::game::infinite_zoom::ENGINE_MAX
}
