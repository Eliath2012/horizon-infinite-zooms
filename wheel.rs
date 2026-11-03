// Claim scroll input while zooming so hotbar selection stays unchanged.
fn infinite_zoom_wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    use crate::game::infinite_zoom as iz;
    if game.native_on(iz::ID) && iz::wheel(steps) {
        *claimed = true;
    }
}
