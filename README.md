# Infinite Zoom for Horizon Nightly

Experimental native Horizon mod with **up to 1,000,000x requested magnification**, independently configurable from the official Zoom mod. This version targets Horizon **Nightly 2026-11-07** (source `2ebf00a`, Minecraft 26.3). It may require rebuilding on newer Nightlies.

## Controls
- **Hold C** to enter zoom (rebind in Options > Mods > Infinite Zoom).
- **Scroll while holding C** to zoom in/out without changing hotbar selection.
- Settings: starting factor 2–256x, maximum 100 / 1,000 / 10,000 / 100,000 / 1,000,000x, wheel multiplier, smooth zoom, precision turning, zoom HUD.
- **v1.2.0:** Smooth Zoom now has an engine-level low-FOV fix, not just a scrolling-number fix. If you are still using v1.1.0, switch **Smooth Zoom OFF** as a temporary workaround. At extreme factors the image may be limited by geometry precision or available loaded detail.

## Why this is different from the 10x zoom mod

Horizon's `Game::native_zoom_now` restricts zoom to 10x and turning sensitivity to >=0.05. This bundle has a **real cancellable HEAD** on that method, active only while this mod's zoom is held. A second precise `MODIFY_CONSTANT` changes the minimum *accepted* FOV in `Game::fov_degrees` from 1 degree to 0.00001 degree. **v1.2.0 fixes the actual smooth-zoom bottleneck**: Horizon's `Camera::tick` also clamps the smoothed FOV modifier to a minimum of `0.1`, which makes the image freeze at roughly 10x while the mod counter continues increasing. A third targeted camera mixin lowers that floor to `0.0000001`. Otherwise ordinary zoom behavior remains unchanged.

This only magnifies rendered geometry, **not distant unloaded terrain**. 100,000x and 1,000,000x are experimental due to graphics precision, not guaranteed visually usable. Extremely narrow FOV may also reveal other renderer limitations.

## Install

1. On this branch's **Actions** tab, open the `Package Infinite Zoom` run and download the `infinite-zoom-hnmod` artifact.
2. Extract that Actions artifact. Inside is `infinite_zoom-1.2.0.hnmod`.
3. Replace the old `infinite_zoom-1.1.0.hnmod` in the same local native-mod installation location you used previously with the new `infinite_zoom-1.2.0.hnmod`. Run Horizon's **Native Build** again, then enable the mod in Options > Mods.
4. Disable the official Zoom mod, or leave it assigned to a different key, to avoid overlapping zoom controls.

The GitHub Actions package is only a **source bundle**. It is not a compiled standalone game binary. Full in-game operation on a specific Nightly requires building the patched Horizon client and testing the camera.

## Source and testing

Existing `main` branch code is preserved. All changes are on this feature branch. Native mod bundle helpers are in the repository root, which the packaging workflow lays out under `mixins/infinite_zoom/` inside the .hnmod. The extra `engine_cap.rs` and `engine_turn.rs` are older unused experiments and do **not** provide the runtime patch.

GitHub Actions checks pure Rust zoom controls, validates mixin injection against the pinned Nightly source, compiles patched `horizon-client`, and packages the `.hnmod` if all steps pass. Passing compilation does not prove an actual rendered view at every zoom level; please check visually with Smooth Zoom ON and OFF.

Designed for your own authorized Minecraft installation. No Minecraft files are included.
