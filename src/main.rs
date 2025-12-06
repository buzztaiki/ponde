use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use crate::app::App;
use crate::config::Config;
use crate::sink_device::SinkDevice;

mod app;
mod config;
mod default_libinput_interface;
mod device_fd;
mod errors;
mod inspect_event;
mod sink_device;
mod sink_event;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    config_file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::new().default_filter_or(log::Level::Info.as_str()),
    )
    .init();

    let args = Args::parse();
    let config = Config::load(&args.config_file).context("failed to load config")?;
    let sink_device = SinkDevice::create("ponde").context("failed to create sink device")?;
    let mut app = App::new(&config, sink_device);
    app.main_loop()?;
    Ok(())
}
