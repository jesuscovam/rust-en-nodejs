extern crate napi_derive;
mod fibonacci;
use fibonacci::fibonacci;
use napi::{Env, JsObject, Result};
use napi_derive::module_exports;

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
    exports.create_named_method("fibonacci", fibonacci)?;
    exports.set_named_property("DEFAULT_VALUE", env.create_int64(100)?)?;
    Ok(())
}
