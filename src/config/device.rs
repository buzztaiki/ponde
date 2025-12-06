use std::collections::HashMap;

use serde::Deserialize;

use crate::config::scroll_factor::ScrollFactorPair;
use crate::errors::Error;

use super::accel_profile::AccelProfile;
use super::button::Button;
use super::device_info::DeviceInfo;
use super::match_rule::MatchRule;

#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct Device {
    pub match_rule: MatchRule,

    /// Sets the pointer acceleration profile to the given profile. Permitted values are `adaptive`, `flat`.  Not all devices support this option or all profiles. If a profile is unsupported, the default profile for this device is used. For a description on the profiles and their behavior, see the libinput documentation.
    pub accel_profile: Option<AccelProfile>,

    /// Sets the pointer acceleration speed within the range [-1, 1]
    pub accel_speed: Option<f64>,

    /// Sets the logical button mapping for this device.
    #[serde(default)]
    pub button_mapping: HashMap<Button, Button>,

    /// Enables left-handed button orientation, i.e. swapping left and right buttons.
    pub left_handed: Option<bool>,

    /// Enables middle button emulation. When enabled, pressing the left and right buttons simultaneously produces a middle mouse button click.
    pub middle_emulation: Option<bool>,

    /// Enables or disables natural scrolling behavior.
    pub natural_scrolling: Option<bool>,

    /// Sets the rotation angle of the device to the given angle, in degrees clockwise. The angle must be between 0 (inclusive) and 360 (exclusive).
    pub rotation_angle: Option<u32>,

    /// Designates a button as scroll button. If the button is logically down, x/y axis movement is converted into scroll events.
    pub scroll_button: Option<Button>,

    /// Enables or disables the scroll button lock. If enabled, the `scroll_button` is considered logically down after the first click and remains down until the second click of that button. If disabled (the default), the `scroll_button` is considered logically down while held down and up once physically released.
    pub scroll_button_lock: Option<bool>,

    /// Sets the wheel scroll speed factor. You can control wheel scroll speed to change this value.
    #[serde(default)]
    pub wheel_scroll_factor: ScrollFactorPair,

    /// Sets the motion scroll (with scroll_button) speed factor. You can control motion scroll speed to change this value.
    #[serde(default)]
    pub motion_scroll_factor: ScrollFactorPair,
}

impl Device {
    pub fn apply_to(&self, device: &mut input::Device) -> Result<(), Error> {
        if let Some(x) = self.accel_profile {
            device.config_accel_set_profile(x.into())?;
        }

        if let Some(x) = self.accel_speed {
            device.config_accel_set_speed(x)?;
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

    pub fn map_button(&self, button: Button) -> Button {
        self.button_mapping.get(&button).copied().unwrap_or(button)
    }

    pub fn matches(&self, device_info: &DeviceInfo) -> bool {
        self.match_rule.matches(device_info)
    }
}
