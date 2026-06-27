/// No-op on native targets; mirrors the WASM API that disables persistent timer
/// overrides. Always returns `Ok(())`.
pub fn disable_persistent_timer_overrides() -> Result<(), String> {
    Ok(())
}
