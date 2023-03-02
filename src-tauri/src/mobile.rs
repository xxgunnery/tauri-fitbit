use dotenv::dotenv;

#[tauri::mobile_entry_point]
pub fn main() {
    dotenv().ok();
    super::AppBuilder::new().run();
}
