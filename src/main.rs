mod app_service;
mod layout;
mod message_service;
mod native;
mod ui;
mod utils;

use app_service::AppService;
use clap::Parser;
use layout::assets::LayoutAssets;
use message_service::MessageService;
use native::VirtualKeyboard;
use tracing::info;
use utils::ProgramArgs;

fn main() {
    tracing_subscriber::fmt::init();

    let args = ProgramArgs::parse();
    info!("Message: {:?}", args);

    let default_layout = LayoutAssets::get_default_60_percent_layout();

    let keyboard = VirtualKeyboard::new();

    let app_service = AppService::new(keyboard, default_layout);
    app_service.run();

    // let message_service = MessageService::new();
    // message_service.run();
}
