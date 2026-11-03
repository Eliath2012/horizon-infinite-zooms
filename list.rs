// Register an independent entry in the Options > Mods native-mod menu.
fn infinite_zoom_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::infinite_zoom as iz;
    mods.push(crate::game::native_hooks::NativeMod {
        id: iz::ID.into(),
        title: format!("Infinite Zoom {}", iz::VERSION),
        description: iz::DESCRIPTION.into(),
        keys: vec![[iz::KEY.into(), "Zoom key".into(), iz::DEFAULT_KEY.into()]],
    });
}
