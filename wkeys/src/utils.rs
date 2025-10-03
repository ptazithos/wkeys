use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum MessageEnum {
    Close,
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct ProgramArgs {
    #[arg(short, long)]
    pub message: Option<MessageEnum>,

    #[arg(short, long, help = "Path to a custom layout file (.toml)")]
    pub layout: Option<String>,

    #[arg(short, long, help = "Path to a custom style file (.css)")]
    pub style: Option<String>,

    #[arg(short = 'i', long, default_value_t = 40, help = "Height of a key")]
    pub height: i32,
}
