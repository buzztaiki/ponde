use serde::Deserialize;

use super::device_info::DeviceInfo;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct MatchRule {
    pub name: String,
}

impl MatchRule {
    pub fn matches(&self, device_info: &DeviceInfo) -> bool {
        device_info.is_mouse() && device_info.name == self.name
    }
}
