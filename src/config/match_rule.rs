use serde::Deserialize;

use super::device_info::DeviceInfo;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct MatchRule {
    pub name: String,
}

impl MatchRule {
    pub fn matches(&self, device: &DeviceInfo) -> bool {
        device.pointer && !device.gesture && device.name == self.name
    }
}
