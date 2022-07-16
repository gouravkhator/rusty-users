mod api_handlers;
mod render_ui;

use render_ui::UsersApp;
use tokio;

use api_handlers::UsersError;

use eframe::{run_native, NativeOptions, egui};

#[tokio::main]
async fn main() -> Result<(), UsersError> {
    let window_options = NativeOptions {
        min_window_size: Some(egui::vec2(320., 100.)),
        ..Default::default()
    };

    let users_app = UsersApp::new().await;
    
    run_native(
        "Users App",
        window_options,
        Box::new(|_cc| Box::new(users_app)),
    );
}
