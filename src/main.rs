mod app_service;
mod layout;
mod message_service;
mod native;
mod ui;

use app_service::AppService;
use layout::assets::LayoutAssets;
use message_service::MessageService;
use native::VirtualKeyboard;
use single_instance::SingleInstance;

fn main() {
    tracing_subscriber::fmt::init();

    let instance = SingleInstance::new("net.pithos.wkeys").unwrap();

    if instance.is_single() {
        let default_layout = LayoutAssets::get_default_60_percent_layout();

        let keyboard = VirtualKeyboard::new();

        let app_service = AppService::new(keyboard, default_layout);
        app_service.run();
    } else {
        let message_service = MessageService::new();
        message_service.run();
    }
}
