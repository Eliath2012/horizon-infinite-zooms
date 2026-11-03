// Rebind through Options > Mods > Infinite Zoom.
fn infinite_zoom_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::infinite_zoom as iz;
    if !game.native_on(iz::ID) {
        iz::release();
        return;
    }
    let bound = game.native_mod_key(iz::ID, iz::KEY, iz::DEFAULT_KEY);
    if key != bound {
        return;
    }
    let playing = game.native_playing();
    let game = &*game;
    iz::key(playing, pressed, |name| game.native_setting(iz::ID, name));
}
