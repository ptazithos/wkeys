mod app_service;
mod ipc;
mod layout;
mod message_service;
mod native;
mod ui;
mod utils;

use app_service::AppService;
use clap::Parser;
use ipc::IPCHandle;
use layout::assets::LayoutAssets;
use message_service::MessageService;
use native::VirtualKeyboard;
use tracing::info;
use utils::ProgramArgs;

fn main() {
    tracing_subscriber::fmt::init();

    let args = ProgramArgs::parse();
    info!("Message: {:?}", args);

    let ipc_handle = IPCHandle::init();

    if ipc_handle.is_single_instance() {
        info!("Starting app service.");

        ctrlc::set_handler(move || {
            IPCHandle::clean_up();
            std::process::exit(0);
        })
        .unwrap();

        let default_layout = LayoutAssets::get_default_60_percent_layout();

        let keyboard = VirtualKeyboard::new();

        AppService::new(keyboard, default_layout).run();
    } else {
        info!("Starting message service.");
        MessageService::new().run();
    }
}
