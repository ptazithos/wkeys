mod ipc;
mod layout;
mod native;
mod service;
mod ui;
mod utils;

use clap::Parser;
use ipc::IPC;
use layout::assets::LayoutAssets;
use native::VirtualKeyboard;
use service::client::MessageService;
use service::host::AppService;
use tracing::info;
use utils::ProgramArgs;

fn main() {
    tracing_subscriber::fmt::init();

    let args = ProgramArgs::parse();
    info!("Message: {:?}", args);

    let ipc = IPC::init();

    if ipc.is_single_instance() {
        info!("Starting app service.");

        ctrlc::set_handler(move || {
            IPC::clean_up();
            std::process::exit(0);
        })
        .unwrap();

        let default_layout = LayoutAssets::get_default_60_percent_layout();

        let keyboard = VirtualKeyboard::new();

        AppService::new(keyboard, ipc, default_layout).run();
        info!("App Service Exiting.");
    } else {
        info!("Starting message service.");
        MessageService::new(ipc).run();
        info!("Message service exiting.");
    }

    info!("Exited");
}
