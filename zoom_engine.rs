// Run before Horizon's built-in native_zoom_now 10x clamp.
// Only intercept while our mod is enabled AND the zoom key is held.
fn infinite_zoom_engine_override(
    game: &crate::game::Game<'_>,
) -> ::core::ops::ControlFlow<Option<NativeZoom>> {
    use crate::game::infinite_zoom as iz;
    use crate::game::native_hooks::NativeZoom;

    if !game.native_on(iz::ID) || !game.native_playing() || game.scoping() {
        return ::core::ops::ControlFlow::Continue(());
    }
    let Some((factor, smooth, turn)) = iz::view() else {
        return ::core::ops::ControlFlow::Continue(());
    };
    if !factor.is_finite() || factor <= 1.0 {
        return ::core::ops::ControlFlow::Continue(());
    }
    // Do not let NaN, infinity or impossible bounds reach the renderer.
    let safe_factor = factor.clamp(1.0, iz::ENGINE_MAX);
    let safe_turn = if turn.is_finite() {
        turn.clamp(0.000001, 1.0)
    } else {
        1.0
    };
    ::core::ops::ControlFlow::Break(Some(NativeZoom {
        factor: safe_factor,
        smooth,
        turn: safe_turn,
    }))
}
