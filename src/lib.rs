mod moss_definitions;
pub use moss_definitions::*;

use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[plugin_fn]
pub unsafe fn moss_register(Json(state): Json<MossState>) -> FnResult<ExtensionInfo> {
    Ok(ExtensionInfo { files: [].to_vec() })
}

#[plugin_fn]
pub unsafe fn extension_loop(Json(state): Json<MossState>) -> FnResult<()> {
    Ok(())
}

#[plugin_fn]
pub fn unregister() -> FnResult<()> {
    Ok(())
}
