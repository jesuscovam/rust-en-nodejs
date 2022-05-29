use napi::{CallContext, JsNumber};
use napi_derive::js_function;

#[js_function(1)]
pub fn fibonacci(ctx: CallContext) -> Result<JsNumber, napi::Error> {
    let n = ctx.get::<JsNumber>(0)?.try_into()?;
    ctx.env.create_int64(fibonacci_native(n))
}

#[inline(always)]
fn fibonacci_native(n: i64) -> i64 {
    match n {
        1 | 2 => 1,
        _ => fibonacci_native(n - 1) + fibonacci_native(n - 2),
    }
}
