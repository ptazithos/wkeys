use applet::Applet;
use cosmic::iced::Result;

mod applet;
fn main() -> Result {
    tracing_subscriber::fmt().with_writer(non_blocking).init();
    cosmic::applet::run::<Applet>(())?;

    Ok(())
}
