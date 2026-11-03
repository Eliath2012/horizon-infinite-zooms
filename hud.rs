// Minimal top-right zoom readout, only while the zoom key is held.
fn infinite_zoom_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::infinite_zoom as iz;
    use horizon_client::extras::Corner;
    if !game.native_on(iz::ID) || !game.native_playing() {
        return;
    }
    let Some(label) = iz::hud_label() else {
        return;
    };
    let width = game.native_text_width(&label) + 10.0;
    let (x, y) = corners.place(Corner::TopRight, width, 16.0);
    game.native_fill(batch, [x, y, width, 16.0], 0xC018_1428);
    game.native_text(batch, &label, [x + 5.0, y + 4.0], 0xC3A6FF, true);
}
