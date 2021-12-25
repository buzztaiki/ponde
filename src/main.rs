use std::env;

use app::App;
use config::{Config, DeviceConfig};
use sink_device::SinkDevice;

mod app;
mod config;
mod default_libinput_interface;
mod device_fd;
mod errors;
mod sink_device;
mod sink_event;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let (_name, args) = args.split_first().unwrap();

    let mut config = Config::default();
    for name in args.iter().map(|x| x.to_string()) {
        config.devices.push(DeviceConfig {
            match_rule: config::DeviceMatchRule { name },
        })
    }

    let sink_device = SinkDevice::create("ponde")?;
    let mut app = App::new(&config, sink_device);
    app.event_loop()?;
    Ok(())
}
