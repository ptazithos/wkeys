use applet::Applet;
use cosmic::iced::Result;

mod applet;
fn main() -> Result {
    cosmic::applet::run::<Applet>(())?;

    Ok(())
}
