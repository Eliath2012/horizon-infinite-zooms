// Save settings when clicked in the native-mod UI.
fn infinite_zoom_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::infinite_zoom as iz;
    if mod_id != iz::ID {
        return;
    }
    let mut cfg = iz::settings(|key| game.native_setting(iz::ID, key));
    cfg.click(button);
    iz::set_settings(cfg);
    for (key, value) in cfg.stored() {
        game.native_set_setting(iz::ID, key, &value);
    }
}
