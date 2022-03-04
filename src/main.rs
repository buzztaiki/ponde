use std::env;
use std::path::Path;
use std::process::exit;

use anyhow::Context;

use crate::app::App;
use crate::config::Config;
use crate::sink_device::SinkDevice;

mod app;
mod config;
mod default_libinput_interface;
mod device_fd;
mod errors;
mod sink_device;
mod sink_event;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let (name, args) = args.split_first().unwrap();
    if args.len() != 1 {
        eprintln!("usage: {} <config-file>", name);
        exit(1);
    }

    let config = Config::load(Path::new(&args[0])).context("failed to load config")?;
    let sink_device = SinkDevice::create("ponde").context("failed to create sink device")?;
    let mut app = App::new(&config, sink_device);
    app.main_loop()?;
    Ok(())
}
