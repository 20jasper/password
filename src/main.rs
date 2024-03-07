mod app;
mod panic;
pub mod password;
pub mod tui;

use app::App;

fn main() -> color_eyre::Result<()> {
    panic::install_hooks()?;

    App::default().run(&mut tui::init()?)?;
    tui::restore()?;

    Ok(())
}
