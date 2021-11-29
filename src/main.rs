use app::App;

mod app;
mod default_libinput_interface;
mod device_fd;

fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    app.event_loop()?;
    Ok(())
}
