use crate::concurrency::SendWrapper;
use js_sys::Function;
use std::sync::OnceLock;
use wasm_bindgen::prelude::{JsCast, JsValue};
use web_sys::{Window, WorkerGlobalScope};

pub(crate) enum WebGlobalScope {
    Window(Window),
    WorkerGlobalScope(WorkerGlobalScope),
}

impl WebGlobalScope {
    pub(crate) fn clear_interval(&self, interval_id: i32) {
        match self {
            WebGlobalScope::Window(window) => window.clear_interval_with_handle(interval_id),
            WebGlobalScope::WorkerGlobalScope(scope) => {
                scope.clear_interval_with_handle(interval_id)
            }
        }
    }

    pub(crate) fn set_interval(
        &self,
        callback: &Function,
        delay_milliseconds: i32,
    ) -> Result<i32, JsValue> {
        match self {
            WebGlobalScope::Window(window) => window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    callback,
                    delay_milliseconds,
                ),
            WebGlobalScope::WorkerGlobalScope(scope) => scope
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    callback,
                    delay_milliseconds,
                ),
        }
    }

    pub(crate) fn set_timeout(
        &self,
        callback: &Function,
        delay_milliseconds: i32,
    ) -> Result<i32, JsValue> {
        match self {
            WebGlobalScope::Window(window) => window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback,
                    delay_milliseconds,
                ),
            WebGlobalScope::WorkerGlobalScope(scope) => scope
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback,
                    delay_milliseconds,
                ),
        }
    }
}

fn get_web_global_scope() -> Result<WebGlobalScope, JsValue> {
    let global = js_sys::global();

    if js_sys::eval(
        "typeof WorkerGlobalScope !== 'undefined' && self instanceof WorkerGlobalScope",
    )?
    .as_bool()
    .unwrap_or(false)
    {
        Ok(global
            .dyn_into::<WorkerGlobalScope>()
            .map(WebGlobalScope::WorkerGlobalScope)?)
    } else if js_sys::eval("typeof Window !== 'undefined' && self instanceof Window")?
        .as_bool()
        .unwrap_or(false)
    {
        Ok(global.dyn_into::<Window>().map(WebGlobalScope::Window)?)
    } else {
        Err("failed to get Window or WorkerGlobalScope".into())
    }
}

pub(crate) fn web_global_scope() -> &'static SendWrapper<WebGlobalScope> {
    static INSTANCE: OnceLock<SendWrapper<WebGlobalScope>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        SendWrapper::new(get_web_global_scope().expect("failed to get Window or WorkerGlobalScope"))
    })
}

pub(crate) fn clear_interval(interval_id: i32) {
    web_global_scope().clear_interval(interval_id)
}

pub(crate) fn set_interval(callback: &Function, delay_milliseconds: i32) -> i32 {
    web_global_scope()
        .set_interval(callback, delay_milliseconds)
        .expect("failed to call setInterval in web environment")
}

pub(crate) fn set_timeout(callback: &Function, delay_milliseconds: i32) -> i32 {
    web_global_scope()
        .set_timeout(callback, delay_milliseconds)
        .expect("failed to call setTimeout in web environment")
}
