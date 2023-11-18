#![windows_subsystem = "windows"]
mod data;
mod send_and_get;

mod uiapp;
mod ui_components;
fn main() {
  //  ui_components::login::training();

    let a: bool = uiapp::ui_run();
}
