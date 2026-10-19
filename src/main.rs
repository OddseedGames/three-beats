mod app;
mod audio;
mod browser;
mod input;
mod layout;
mod view;

use app::config;

#[macroquad::main(config)]
async fn main() {
    app::run().await;
}
