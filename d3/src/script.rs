use workflow_dom::inject::*;
use workflow_dom::result::Result;

static mut D3_LOADED: bool = false;

/// Injects the bundled D3.js library into the DOM, loading it only once
/// for the lifetime of the page (subsequent calls are no-ops).
pub async fn load() -> Result<()> {
    if unsafe { D3_LOADED } {
        return Ok(());
    }

    let d3_js = include_bytes!("../extern/resources/d3.v7.min.js");
    inject_blob(Content::Script(None, d3_js)).await?;

    unsafe { D3_LOADED = true };

    Ok(())
}
