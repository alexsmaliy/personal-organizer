mod app;
mod components;
mod misc;
mod server;
mod types;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  console_error_panic_hook::set_once();
  leptos::mount_to_body(app::Omark);
}
