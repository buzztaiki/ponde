use std::path::Path;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::errors::Error;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub devices: Vec<DeviceConfig>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let f = std::fs::File::open(&path)?;
        let config = serde_yaml::from_reader(&f)?;
        Ok(config)
    }

    pub fn matched_device(&self, device: &MatchDevice) -> Option<&DeviceConfig> {
        if device.pointer && !device.gesture {
            self.devices.iter().find(|x| x.match_rule.matches(device))
        } else {
            None
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct DeviceConfig {
    pub match_rule: DeviceMatchRule,

    /// Sets the pointer acceleration profile to the given profile. Permitted values are `adaptive`, `flat`.  Not all devices support this option or all profiles. If a profile is unsupported, the default profile for this device is used. For a description on the profiles and their behavior, see the libinput documentation.
    pub accel_profile: Option<AccelProfile>,

    /// Sets the pointer acceleration speed within the range [-1, 1]
    pub accel_speed: Option<f64>,

    /// Sets  the logical button mapping for this device, see XSetPointerMapping(3). The string must be a space-separated list of button mappings in the order of the logical buttons on the device, starting with button 1.  The default mapping is "1 2 3 ... 32". A mapping of 0 deactivates the button. Multiple buttons can have the same mapping.  Invalid mapping strings are discarded and the default mapping is used for all buttons. Buttons not specified in the user's mapping use the default mapping. See section BUTTON MAPPING for more details.
    pub button_mapping: Option<Vec<u8>>,

    /// Sets "drag lock buttons" that simulate a button logically down even when it has been physically released. To logically release a locked button, a second click of the same button is required.
    /// If the option is a single button number, that button acts as the "meta" locking button for the next button number. See section BUTTON DRAG LOCK for details.
    /// If the option is a list of button number pairs, the first number of each number pair is the lock button, the second number the logical button number to be locked. See section BUTTON DRAG LOCK for details.
    /// For both meta and button pair configuration, the button numbers are device button numbers, i.e. the ButtonMapping applies after drag lock.
    pub drag_lock_buttons: Option<Vec<u8>>,

    /// Disables horizontal scrolling. When disabled, this driver will discard any horizontal scroll events from libinput. Note that this does not disable horizontal scrolling, it merely discards the horizontal axis from any scroll events.
    pub horizontal_scrolling: Option<bool>,

    /// Enables left-handed button orientation, i.e. swapping left and right buttons.
    pub left_handed: Option<bool>,

    /// Enables middle button emulation. When enabled, pressing the left and right buttons simultaneously produces a middle mouse button click.
    pub middle_emulation: Option<bool>,

    /// Enables or disables natural scrolling behavior.
    pub natural_scrolling: Option<bool>,

    /// Sets the rotation angle of the device to the given angle, in degrees clockwise. The angle must be between 0.0 (inclusive) and 360.0 (exclusive).
    pub rotation_angle: Option<u32>,

    /// Designates a button as scroll button. If the button is logically down, x/y axis movement is converted into scroll events.
    pub scroll_button: Option<Button>,

    /// Enables or disables the scroll button lock. If enabled, the ScrollButton is considered logically down after the first click and remains down until the second click of that button. If disabled (the default), the ScrollButton button is considered logically down while held down and up once physically released.
    pub scroll_button_lock: Option<bool>,
}

impl DeviceConfig {
    pub fn apply_to(&self, device: &mut input::Device) -> Result<(), Error> {
        if let Some(x) = self.accel_profile {
            device.config_accel_set_profile(x.into())?;
        }

        if let Some(x) = self.accel_speed {
            device.config_accel_set_speed(x)?;
        }

        if let Some(_x) = &self.button_mapping {
            // TODO
        }

        if let Some(_x) = &self.drag_lock_buttons {
            // TODO
        }

        if let Some(_x) = self.horizontal_scrolling {
            // TODO
        }

        if let Some(x) = self.left_handed {
            device.config_left_handed_set(x)?;
        }

        if let Some(x) = self.middle_emulation {
            device.config_middle_emulation_set_enabled(x)?;
        }

        if let Some(x) = self.natural_scrolling {
            device.config_scroll_set_natural_scroll_enabled(x)?;
        }

        if let Some(x) = self.rotation_angle {
            device.config_rotation_set_angle(x)?;
        }

        if let Some(x) = self.scroll_button {
            device.config_scroll_set_button(x.code().into())?;
            device.config_scroll_set_method(input::ScrollMethod::OnButtonDown)?;
        }

        if let Some(x) = self.scroll_button_lock {
            device.config_scroll_set_button_lock(if x {
                input::ScrollButtonLockState::Enabled
            } else {
                input::ScrollButtonLockState::Disabled
            })?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccelProfile {
    Adaptive,
    Flat,
}

impl From<AccelProfile> for input::AccelProfile {
    fn from(x: AccelProfile) -> Self {
        match x {
            AccelProfile::Adaptive => input::AccelProfile::Adaptive,
            AccelProfile::Flat => input::AccelProfile::Flat,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Button(evdev::Key);

impl<'de> Deserialize<'de> for Button {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        evdev::Key::from_str(&s)
            .ok()
            .filter(|_| s.starts_with("BTN_"))
            .map(Self)
            .ok_or_else(|| serde::de::Error::custom(format!("unexpected button value {}", s)))
    }
}

impl Button {
    pub fn code(&self) -> u16 {
        self.0.code()
    }
}

#[derive(Debug, Default, Deserialize)]
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
            ..Default::default()
        });
        assert!(config.matched_device(&device).is_none());

        config.devices.push(DeviceConfig {
            match_rule: DeviceMatchRule {
                name: "moo".to_string(),
            },
            ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
            }],
        };
        assert!(config.matched_device(&device).is_none());
    }
}
