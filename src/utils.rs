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
}
