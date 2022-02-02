mod accel_profile;
mod button;
mod device;
mod device_info;
mod match_rule;

use std::path::Path;

use serde::Deserialize;

use crate::errors::Error;

use self::device::Device;
use self::device_info::DeviceInfo;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub devices: Vec<Device>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let f = std::fs::File::open(&path)?;
        let config = serde_yaml::from_reader(&f)?;
        Ok(config)
    }

    pub fn matched_device(&self, device_info: &DeviceInfo) -> Option<&Device> {
        self.devices.iter().find(|x| x.matches(device_info))
    }
}

#[cfg(test)]
mod tests {
    use crate::config::device::Device;
    use crate::config::device_info::DeviceInfo;

    use super::*;

    #[test]
    fn test_empty() {
        let device_info = DeviceInfo {
            name: "moo".to_string(),
            pointer: true,
            gesture: false,
        };

        let config = Config { devices: vec![] };
        assert!(config.matched_device(&device_info).is_none());
    }

    #[test]
    fn test_found_device() {
        let device_info = DeviceInfo {
            name: "moo".to_string(),
            pointer: true,
            gesture: false,
        };

        let mut device_config = Device::default();
        device_config.match_rule.name = "moo".to_string();
        let config = Config {
            devices: vec![device_config],
        };
        assert!(config.matched_device(&device_info).is_some());
    }
}
