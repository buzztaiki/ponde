#[derive(Debug, Default)]
pub struct Config {
    pub devices: Vec<DeviceConfig>,
}

impl Config {
    pub fn matched_device(&self, device: &MatchDevice) -> Option<&DeviceConfig> {
        if device.pointer && !device.gesture {
            self.devices.iter().find(|x| x.match_rule.matches(device))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct DeviceConfig {
    pub match_rule: DeviceMatchRule,
}

#[derive(Debug, Default)]
pub struct DeviceMatchRule {
    pub name: String,
}

impl DeviceMatchRule {
    pub fn matches(&self, device: &MatchDevice) -> bool {
        device.name == self.name
    }
}

pub struct MatchDevice {
    pub name: String,
    pub pointer: bool,
    pub gesture: bool,
}

impl From<&input::Device> for MatchDevice {
    fn from(x: &input::Device) -> Self {
        MatchDevice {
            name: x.name().to_string(),
            pointer: x.has_capability(input::DeviceCapability::Pointer),
            gesture: x.has_capability(input::DeviceCapability::Gesture),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_device() {
        let device = MatchDevice {
            name: "moo".to_string(),
            pointer: true,
            gesture: false,
        };

        let mut config = Config { devices: vec![] };
        assert!(config.matched_device(&device).is_none());

        config.devices.push(DeviceConfig {
            match_rule: DeviceMatchRule {
                name: "woo".to_string(),
            },
        });
        assert!(config.matched_device(&device).is_none());

        config.devices.push(DeviceConfig {
            match_rule: DeviceMatchRule {
                name: "moo".to_string(),
            },
        });
        assert!(config.matched_device(&device).is_some());
        assert_eq!(
            config.matched_device(&device).unwrap().match_rule.name,
            "moo".to_string()
        );
    }

    #[test]
    fn test_non_pointer_device() {
        let device = MatchDevice {
            name: "moo".to_string(),
            pointer: false,
            gesture: false,
        };

        let config = Config {
            devices: vec![DeviceConfig {
                match_rule: DeviceMatchRule {
                    name: "moo".to_string(),
                },
            }],
        };
        assert!(config.matched_device(&device).is_none());
    }

    #[test]
    fn test_gesture_device() {
        let device = MatchDevice {
            name: "moo".to_string(),
            pointer: true,
            gesture: true,
        };

        let config = Config {
            devices: vec![DeviceConfig {
                match_rule: DeviceMatchRule {
                    name: "moo".to_string(),
                },
            }],
        };
        assert!(config.matched_device(&device).is_none());
    }
}
