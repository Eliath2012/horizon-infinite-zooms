// Six configurable settings, rendered as the native mod's clickable buttons.
fn infinite_zoom_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::infinite_zoom as iz;
    if mod_id == iz::ID {
        let game = &*game;
        buttons.extend(iz::settings(|key| game.native_setting(iz::ID, key)).labels());
    }
}
