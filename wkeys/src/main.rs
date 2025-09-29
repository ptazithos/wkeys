mod config;
mod ipc;
mod layout;
mod native;
mod service;
mod ui;
mod utils;

use clap::Parser;
use config::AppConfig;
use ipc::IPC;
use layout::parse::LayoutDefinition;
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
            info!("Received SIGINT. Exiting.");
            IPC::clean_up();
            std::process::exit(0);
        })
        .unwrap();

        let app_config = AppConfig::new(args.layout.clone(), args.style.clone());
        let user_layout = LayoutDefinition::from_toml(&app_config.get_layout_file_content());
        let user_style = app_config.get_css_file_content();

        let keyboard = VirtualKeyboard::new();

        AppService::new(keyboard, ipc, user_layout, user_style).run();
        info!("App Service Exiting.");
    } else {
        info!("Starting message service.");
        MessageService::new(ipc, args).run();
        info!("Message service exiting.");
    }

    info!("Exited");
}
