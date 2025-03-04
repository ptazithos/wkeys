use applet::Applet;
use cosmic::iced::Result;

mod applet;
fn main() -> Result {
    tracing_subscriber::fmt::init();
    cosmic::applet::run::<Applet>(())?;

    Ok(())
}
