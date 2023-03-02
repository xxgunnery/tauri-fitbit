#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use dotenv::dotenv;

pub fn main() {
    dotenv().ok();
    app::AppBuilder::new().run();
}
