// Native zoom effect; first claimant wins (do not compete with other zoom mods).
fn infinite_zoom_view(game: &crate::game::Game<'_>, zoom: &mut Option<crate::game::native_hooks::NativeZoom>) {
    use crate::game::infinite_zoom as iz;
    if zoom.is_none() && game.native_on(iz::ID)
        && let Some((factor, smooth, turn)) = iz::view()
    {
        *zoom = Some(crate::game::native_hooks::NativeZoom { factor, smooth, turn });
    }
}
