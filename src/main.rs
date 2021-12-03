use std::env;

use app::App;
use config::{Config, DeviceConfig};

mod app;
mod config;
mod default_libinput_interface;
mod device_fd;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let (_name, args) = args.split_first().unwrap();

    let mut config = Config::default();
    for name in args.iter().map(|x| x.to_string()) {
        config.devices.push(DeviceConfig {
            match_rule: config::DeviceMatchRule { name },
        })
    }

    let mut app = App::new(&config);
    app.event_loop()?;
    Ok(())
}
