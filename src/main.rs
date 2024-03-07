use password_manager::app::App;
use password_manager::panic;
use password_manager::tui;

fn main() -> color_eyre::Result<()> {
    panic::install_hooks()?;

    App::default().run(&mut tui::init()?)?;
    tui::restore()?;

    Ok(())
}
