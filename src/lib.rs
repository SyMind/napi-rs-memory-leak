#![deny(clippy::all)]

use std::thread::spawn;

use napi::{
  threadsafe_function::{
    ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
  },
  JsFunction,
};

#[macro_use]
extern crate napi_derive;

#[napi(js_name = "QueryEngine")]
pub struct JsQueryEngine {
  tsfn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled>,
}

#[napi]
impl JsQueryEngine {
  #[napi(constructor)]
  pub fn new(callback: JsFunction) -> Self {
    let tsfn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled> = callback
      .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<String>| Ok(vec![ctx.value]))
      .unwrap();
    JsQueryEngine {
      tsfn,
    }
  }

  #[napi]
  pub fn call(&self) {
    let tsfn = self.tsfn.clone();
    spawn(move || {
      tsfn.call(
        Ok("hello".to_string()),
        ThreadsafeFunctionCallMode::Blocking,
      );
    });
  }
}

impl Drop for JsQueryEngine {
  fn drop(&mut self) {
    println!("JsQueryEngine drop")
  }
}
