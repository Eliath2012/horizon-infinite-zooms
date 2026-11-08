// Horizon's Camera::tick clamps eased field-of-view modifiers to 0.1,
// which silently caps Smooth Zoom at ~10x despite the native_zoom hook
// accepting much larger factors.
//
// Only replace that exact low bound in the camera easing path. Normal
// gameplay still targets >= 0.1 and follows its original interpolation.
// FOV value remains positive even at 1,000,000x (70 degrees -> 0.00007 deg).
fn infinite_zoom_easing_lower_bound(old: f32) -> f32 {
    debug_assert!((old - 0.1).abs() < 1e-6);
    0.0000001
}
